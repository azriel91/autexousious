use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    network::simulation::{DeliveryRequirement, TransportResource, UrgencyRequirement},
    shred::{ResourceId, SystemData},
    shrev::ReaderId,
};
use derivative::Derivative;
use derive_new::new;
use log::{debug, error};
use net_model::play::{NetEvent, NetEventChannel, NetMessage};
use network_session_model::play::{
    Session, SessionDevice, SessionDeviceId, SessionDevices, Sessions,
};
use network_session_play::SessionCodeGenerator;
use session_host_model::{
    play::{SessionAcceptResponse, SessionHostRequestParams, SessionRejectResponse},
    SessionHostEvent,
};

/// Limit for number of sessions the server may host;
const SESSION_COUNT_LIMIT: usize = 100;

/// Accepts or rejects session hosting requests, and sends the response to the requester.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionHostResponderSystemDesc))]
pub struct SessionHostResponderSystem {
    /// Reader ID for the `SessionHostEvent` channel.
    #[system_desc(event_channel_reader)]
    session_host_event_rid: ReaderId<NetEvent<SessionHostEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionHostResponderSystemData<'s> {
    /// `SessionHostEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_host_nec: Read<'s, NetEventChannel<SessionHostEvent>>,
    /// `SessionCodeGenerator` resource.
    #[derivative(Debug = "ignore")]
    pub session_code_generator: Write<'s, SessionCodeGenerator>,
    /// `Sessions` resource.
    #[derivative(Debug = "ignore")]
    pub sessions: Write<'s, Sessions>,
    /// `TransportResource` resource.
    #[derivative(Debug = "ignore")]
    pub transport_resource: Write<'s, TransportResource>,
}

impl SessionHostResponderSystem {
    fn handle_session_request(
        session_code_generator: &mut SessionCodeGenerator,
        sessions: &mut Sessions,
        session_host_request_params: &SessionHostRequestParams,
    ) -> SessionHostEvent {
        let SessionHostRequestParams {
            session_device_name,
        } = session_host_request_params;

        if sessions.len() < SESSION_COUNT_LIMIT {
            let session_code = loop {
                let session_code = session_code_generator.generate();
                if !sessions.contains_key(&session_code) {
                    break session_code;
                }
            };

            let session_device_id = SessionDeviceId::new(0); // ID for host
            let session_device = SessionDevice::new(session_device_id, session_device_name.clone());
            let session_devices = SessionDevices::new(vec![session_device]);

            debug!(
                "Hosting new session `{}` by `{}` with id: `{}`.",
                session_code, session_device_name, session_device_id
            );

            let session = Session::new(session_code.clone(), session_devices);

            sessions.insert(session_code, session.clone());

            let session_accept_response = SessionAcceptResponse::new(session, session_device_id);
            SessionHostEvent::SessionAccept(session_accept_response)
        } else {
            debug!(
                "Rejecting request to host new session from `{}`.",
                session_device_name
            );

            SessionHostEvent::SessionReject(SessionRejectResponse::new())
        }
    }
}

impl<'s> System<'s> for SessionHostResponderSystem {
    type SystemData = SessionHostResponderSystemData<'s>;

    fn run(
        &mut self,
        SessionHostResponderSystemData {
            session_host_nec,
            mut session_code_generator,
            mut sessions,
            mut transport_resource,
        }: Self::SystemData,
    ) {
        session_host_nec
            .read(&mut self.session_host_event_rid)
            .filter_map(|session_host_event| {
                if let NetEvent {
                    socket_addr,
                    event: SessionHostEvent::SessionHostRequest(session_host_request_params),
                } = session_host_event
                {
                    Some((*socket_addr, session_host_request_params))
                } else {
                    None
                }
            })
            .map(|(socket_addr, session_host_request_params)| {
                let session_host_event = Self::handle_session_request(
                    &mut session_code_generator,
                    &mut sessions,
                    session_host_request_params,
                );

                (socket_addr, NetMessage::from(session_host_event))
            })
            .for_each(|(socket_addr, net_message)| {
                match bincode::serialize(&net_message) {
                    Ok(payload) => {
                        transport_resource.send_with_requirements(
                            socket_addr,
                            &payload,
                            // None means it uses a default multiplexed stream.
                            //
                            // Suspect if we give it a value, the value will be a "channel" over the
                            // same socket connection.
                            DeliveryRequirement::ReliableOrdered(None),
                            UrgencyRequirement::OnTick,
                        );
                    }
                    Err(e) => {
                        error!(
                            "Failed to serialize `NetMessage::SessionHostEvent`. Error: `{}`.",
                            e
                        );
                    }
                }
            });
    }
}
