#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr};

    use amethyst::{
        ecs::{Read, SystemData, World, WorldExt, WriteExpect},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use net_model::play::{NetData, NetEventChannel};
    use network_session_model::play::SessionStatus;
    use session_lobby_model::SessionLobbyEvent;

    use session_lobby_play::SessionLobbyResponseSystemDesc;

    #[test]
    fn does_nothing_when_no_session_lobby_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                session_lobby_event: None,
            },
            ExpectedParams {
                session_lobby_event: None,
            },
        )
    }

    #[test]
    fn inserts_resources_on_session_accepted() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::HostEstablished,
                session_lobby_event: Some(SessionLobbyEvent::SessionStartNotify),
            },
            ExpectedParams {
                session_lobby_event: Some(SessionLobbyEvent::SessionStartNotify),
            },
        )
    }

    fn run_test(
        SetupParams {
            session_status: session_status_setup,
            session_lobby_event,
        }: SetupParams,
        ExpectedParams {
            session_lobby_event: session_lobby_event_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(<Read<'_, EventChannel<SessionLobbyEvent>> as SystemData>::setup)
            .with_setup(setup_session_lobby_event_reader)
            .with_system_desc(SessionLobbyResponseSystemDesc::default(), "", &[])
            .with_resource(session_status_setup)
            .with_effect(move |world| {
                if let Some(session_lobby_event) = session_lobby_event {
                    let socket_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 1234));
                    world
                        .write_resource::<NetEventChannel<SessionLobbyEvent>>()
                        .single_write(NetData {
                            socket_addr,
                            data: session_lobby_event,
                        });
                }
            })
            .with_assertion(move |world| {
                let (mut session_lobby_event_rid, session_lobby_ec) = world.system_data::<(
                    WriteExpect<'_, ReaderId<SessionLobbyEvent>>,
                    Read<'_, EventChannel<SessionLobbyEvent>>,
                )>();
                let session_lobby_event =
                    session_lobby_ec.read(&mut *session_lobby_event_rid).next();

                assert_eq!(session_lobby_event_expected.as_ref(), session_lobby_event);
            })
            .run()
    }

    fn setup_session_lobby_event_reader(world: &mut World) {
        let session_lobby_event_rid = world
            .write_resource::<EventChannel<SessionLobbyEvent>>()
            .register_reader();
        world.insert(session_lobby_event_rid);
    }

    struct SetupParams {
        session_status: SessionStatus,
        session_lobby_event: Option<SessionLobbyEvent>,
    }

    struct ExpectedParams {
        session_lobby_event: Option<SessionLobbyEvent>,
    }
}
