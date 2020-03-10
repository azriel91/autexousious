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
    use game_input_model::loaded::{PlayerController, PlayerControllers};
    use net_model::play::NetMessageEvent;
    use network_session_model::{
        config::SessionServerConfig,
        play::{SessionCode, SessionDeviceName, SessionStatus},
    };
    use session_join_model::{play::SessionJoinRequestParams, SessionJoinEvent};

    use session_join_play::SessionJoinRequestSystemDesc;

    #[test]
    fn does_nothing_when_no_session_join_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                session_join_event: None,
            },
            ExpectedParams {
                session_status: SessionStatus::None,
                messages: Default::default(),
            },
        )
    }

    #[test]
    fn inserts_resources_on_session_accepted() -> Result<(), Error> {
        let session_join_event = SessionJoinEvent::SessionJoinRequest(SessionJoinRequestParams {
            session_code: SessionCode::new(String::from("abcd")),
            session_device_name: SessionDeviceName::new(String::from("azriel")),
            player_controllers: PlayerControllers::new(vec![PlayerController::new(
                0,
                String::from("p0"),
            )]),
        });

        let payload = Bytes::from(
            bincode::serialize(&NetMessageEvent::SessionJoinEvent(
                session_join_event.clone(),
            ))
            .expect("Failed to serialize `session_join_event`."),
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
                session_join_event: Some(session_join_event),
            },
            ExpectedParams {
                session_status: SessionStatus::JoinRequested {
                    session_code: SessionCode::new(String::from("abcd")),
                },
                messages,
            },
        )
    }

    #[test]
    fn ignores_session_join_request_when_already_requested() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::JoinRequested {
                    session_code: SessionCode::new(String::from("abcd")),
                },
                session_join_event: Some(SessionJoinEvent::SessionJoinRequest(
                    SessionJoinRequestParams {
                        session_code: SessionCode::new(String::from("abcd")),
                        session_device_name: SessionDeviceName::new(String::from("azriel")),
                        player_controllers: PlayerControllers::new(vec![PlayerController::new(
                            0,
                            String::from("p0"),
                        )]),
                    },
                )),
            },
            ExpectedParams {
                session_status: SessionStatus::JoinRequested {
                    session_code: SessionCode::new(String::from("abcd")),
                },
                messages: VecDeque::default(),
            },
        )
    }

    fn run_test(
        SetupParams {
            session_status: session_status_setup,
            session_join_event,
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
                if let Some(session_join_event) = session_join_event {
                    world
                        .write_resource::<EventChannel<SessionJoinEvent>>()
                        .single_write(session_join_event);
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
        session_join_event: Option<SessionJoinEvent>,
    }

    struct ExpectedParams {
        session_status: SessionStatus,
        messages: VecDeque<Message>,
    }
}
