#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Read, SystemData, World, WorldExt, WriteExpect},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_input_model::loaded::{PlayerController, PlayerControllers};
    use net_model::play::NetMessageEvent;
    use network_session_model::play::{SessionCode, SessionDeviceName, SessionStatus};
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
                net_message_event: None,
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

        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                session_join_event: Some(session_join_event.clone()),
            },
            ExpectedParams {
                session_status: SessionStatus::JoinRequested {
                    session_code: SessionCode::new(String::from("abcd")),
                },
                net_message_event: Some(NetMessageEvent::SessionJoinEvent(session_join_event)),
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
                net_message_event: None,
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
            net_message_event: net_message_event_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(<Read<'_, EventChannel<NetMessageEvent>> as SystemData>::setup)
            .with_setup(setup_net_message_event_reader)
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
                let (session_status, mut net_message_event_rid, net_message_ec) = world
                    .system_data::<(
                        Read<'_, SessionStatus>,
                        WriteExpect<'_, ReaderId<NetMessageEvent>>,
                        Read<'_, EventChannel<NetMessageEvent>>,
                    )>();
                let session_status = &*session_status;
                let net_message_event = net_message_ec.read(&mut *net_message_event_rid).next();

                assert_eq!(
                    (
                        &session_status_expected,
                        net_message_event_expected.as_ref()
                    ),
                    (session_status, net_message_event)
                );
            })
            .run()
    }

    fn setup_net_message_event_reader(world: &mut World) {
        let net_message_event_rid = world
            .write_resource::<EventChannel<NetMessageEvent>>()
            .register_reader();
        world.insert(net_message_event_rid);
    }

    struct SetupParams {
        session_status: SessionStatus,
        session_join_event: Option<SessionJoinEvent>,
    }

    struct ExpectedParams {
        session_status: SessionStatus,
        net_message_event: Option<NetMessageEvent>,
    }
}
