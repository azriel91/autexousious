#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Read, SystemData, World, WorldExt, WriteExpect},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use net_model::play::NetMessageEvent;
    use network_session_model::play::{SessionCode, SessionStatus};
    use session_lobby_model::{play::SessionStartRequestParams, SessionLobbyEvent};

    use session_lobby_play::SessionLobbyRequestSystemDesc;

    #[test]
    fn does_nothing_when_no_session_lobby_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::HostEstablished,
                session_lobby_event: None,
            },
            ExpectedParams {
                session_status: SessionStatus::HostEstablished,
                net_message_event: None,
            },
        )
    }

    #[test]
    fn sends_net_message_event_on_session_start_request_when_host_established() -> Result<(), Error>
    {
        let session_lobby_event =
            SessionLobbyEvent::SessionStartRequest(SessionStartRequestParams {
                session_code: SessionCode::new(String::from("abcd")),
            });

        run_test(
            SetupParams {
                session_status: SessionStatus::HostEstablished,
                session_lobby_event: Some(session_lobby_event.clone()),
            },
            ExpectedParams {
                session_status: SessionStatus::HostEstablished,
                net_message_event: Some(NetMessageEvent::SessionLobbyEvent(session_lobby_event)),
            },
        )
    }

    #[test]
    fn sends_net_message_event_on_session_start_request_when_join_established() -> Result<(), Error>
    {
        let session_lobby_event =
            SessionLobbyEvent::SessionStartRequest(SessionStartRequestParams {
                session_code: SessionCode::new(String::from("abcd")),
            });

        run_test(
            SetupParams {
                session_status: SessionStatus::JoinEstablished,
                session_lobby_event: Some(session_lobby_event.clone()),
            },
            ExpectedParams {
                session_status: SessionStatus::JoinEstablished,
                net_message_event: Some(NetMessageEvent::SessionLobbyEvent(session_lobby_event)),
            },
        )
    }

    #[test]
    fn ignores_session_lobby_request_when_session_not_established() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                session_lobby_event: Some(SessionLobbyEvent::SessionStartRequest(
                    SessionStartRequestParams {
                        session_code: SessionCode::new(String::from("abcd")),
                    },
                )),
            },
            ExpectedParams {
                session_status: SessionStatus::None,
                net_message_event: None,
            },
        )
    }

    fn run_test(
        SetupParams {
            session_status: session_status_setup,
            session_lobby_event,
        }: SetupParams,
        ExpectedParams {
            session_status: session_status_expected,
            net_message_event: net_message_event_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(<Read<'_, EventChannel<NetMessageEvent>> as SystemData>::setup)
            .with_setup(setup_net_message_event_reader)
            .with_system_desc(SessionLobbyRequestSystemDesc::default(), "", &[])
            .with_resource(session_status_setup)
            .with_effect(move |world| {
                if let Some(session_lobby_event) = session_lobby_event {
                    world
                        .write_resource::<EventChannel<SessionLobbyEvent>>()
                        .single_write(session_lobby_event);
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
        session_lobby_event: Option<SessionLobbyEvent>,
    }

    struct ExpectedParams {
        session_status: SessionStatus,
        net_message_event: Option<NetMessageEvent>,
    }
}
