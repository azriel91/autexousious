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
    use network_join_model::{
        config::SessionServerConfig, play::SessionJoinRequestParams, NetworkJoinEvent,
    };
    use network_session_model::play::{SessionCode, SessionDeviceName, SessionStatus};

    use network_join_play::SessionJoinRequestSystemDesc;

    #[test]
    fn does_nothing_when_no_network_join_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                network_join_event: None,
            },
            ExpectedParams {
                session_status: SessionStatus::None,
                messages: Default::default(),
            },
        )
    }

    #[test]
    fn inserts_resources_on_session_accepted() -> Result<(), Error> {
        let messages = {
            let session_server_config = SessionServerConfig::default();

            let mut messages = VecDeque::new();
            messages.push_back(Message {
                destination: SocketAddr::from((
                    session_server_config.address,
                    session_server_config.port,
                )),
                payload: Bytes::from("Request to join `abcd` from `azriel`".as_bytes()),
                delivery: DeliveryRequirement::ReliableOrdered(None),
                urgency: UrgencyRequirement::OnTick,
            });

            messages
        };

        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                network_join_event: Some(NetworkJoinEvent::SessionJoinRequest(
                    SessionJoinRequestParams {
                        session_code: SessionCode::new(String::from("abcd")),
                        session_device_name: SessionDeviceName::new(String::from("azriel")),
                    },
                )),
            },
            ExpectedParams {
                session_status: SessionStatus::JoinRequested,
                messages,
            },
        )
    }

    #[test]
    fn ignores_session_join_request_when_already_requested() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::JoinRequested,
                network_join_event: Some(NetworkJoinEvent::SessionJoinRequest(
                    SessionJoinRequestParams {
                        session_code: SessionCode::new(String::from("abcd")),
                        session_device_name: SessionDeviceName::new(String::from("azriel")),
                    },
                )),
            },
            ExpectedParams {
                session_status: SessionStatus::JoinRequested,
                messages: VecDeque::default(),
            },
        )
    }

    fn run_test(
        SetupParams {
            session_status: session_status_setup,
            network_join_event,
        }: SetupParams,
        ExpectedParams {
            session_status: session_status_expected,
            messages: messages_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system_desc(SessionJoinRequestSystemDesc::default(), "", &[])
            .with_resource(session_status_setup)
            .with_effect(move |world| {
                if let Some(network_join_event) = network_join_event {
                    world
                        .write_resource::<EventChannel<NetworkJoinEvent>>()
                        .single_write(network_join_event);
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
        network_join_event: Option<NetworkJoinEvent>,
    }

    struct ExpectedParams {
        session_status: SessionStatus,
        messages: VecDeque<Message>,
    }
}
