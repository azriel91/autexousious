#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, net::SocketAddr};

    use amethyst::{
        ecs::WorldExt,
        network::simulation::{
            DeliveryRequirement, Message, TransportResource, UrgencyRequirement,
        },
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use bytes::Bytes;
    use net_model::play::NetMessage;
    use network_session_model::play::{SessionDeviceName, SessionStatus};
    use session_host_model::{
        config::SessionServerConfig, play::SessionHostRequestParams, SessionHostEvent,
    };

    use session_host_play::SessionHostRequestSystemDesc;

    #[test]
    fn does_nothing_when_no_session_host_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                session_host_event: None,
            },
            ExpectedParams {
                session_status: SessionStatus::None,
                messages: Default::default(),
            },
        )
    }

    #[test]
    fn inserts_resources_on_session_accepted() -> Result<(), Error> {
        let session_host_event = SessionHostEvent::SessionHostRequest(SessionHostRequestParams {
            session_device_name: SessionDeviceName::new(String::from("azriel")),
        });

        let payload = Bytes::from(
            bincode::serialize(&NetMessage::SessionHostEvent(session_host_event.clone()))
                .expect("Failed to serialize `session_host_event`."),
        );

        let messages = {
            let session_server_config = SessionServerConfig::default();

            let mut messages = VecDeque::new();
            messages.push_back(Message {
                destination: SocketAddr::from((
                    session_server_config.address,
                    session_server_config.port,
                )),
                payload,
                delivery: DeliveryRequirement::ReliableOrdered(None),
                urgency: UrgencyRequirement::OnTick,
            });

            messages
        };

        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                session_host_event: Some(session_host_event),
            },
            ExpectedParams {
                session_status: SessionStatus::HostRequested,
                messages,
            },
        )
    }

    #[test]
    fn ignores_session_host_request_when_already_requested() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::HostRequested,
                session_host_event: Some(SessionHostEvent::SessionHostRequest(
                    SessionHostRequestParams {
                        session_device_name: SessionDeviceName::new(String::from("azriel")),
                    },
                )),
            },
            ExpectedParams {
                session_status: SessionStatus::HostRequested,
                messages: VecDeque::default(),
            },
        )
    }

    fn run_test(
        SetupParams {
            session_status: session_status_setup,
            session_host_event,
        }: SetupParams,
        ExpectedParams {
            session_status: session_status_expected,
            messages: messages_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system_desc(SessionHostRequestSystemDesc::default(), "", &[])
            .with_resource(session_status_setup)
            .with_effect(move |world| {
                if let Some(session_host_event) = session_host_event {
                    world
                        .write_resource::<EventChannel<SessionHostEvent>>()
                        .single_write(session_host_event);
                }
            })
            .with_assertion(move |world| {
                let session_status = world.read_resource::<SessionStatus>();
                let session_status = &*session_status;
                let transport_resource = world.read_resource::<TransportResource>();
                let messages = transport_resource.get_messages();

                assert_eq!(
                    (&session_status_expected, &messages_expected),
                    (session_status, messages)
                );
            })
            .run()
    }

    struct SetupParams {
        session_status: SessionStatus,
        session_host_event: Option<SessionHostEvent>,
    }

    struct ExpectedParams {
        session_status: SessionStatus,
        messages: VecDeque<Message>,
    }
}
