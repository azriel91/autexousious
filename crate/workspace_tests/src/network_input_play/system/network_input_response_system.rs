#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr};

    use amethyst::{
        ecs::{Read, SystemData, World, WorldExt, WriteExpect},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_input_model::{
        config::{ControlAction, PlayerActionControl},
        GameInputEvent,
    };
    use net_model::play::{NetData, NetEventChannel};
    use network_session_model::play::SessionStatus;

    use network_input_play::NetworkInputResponseSystemDesc;

    #[test]
    fn does_nothing_when_no_game_input_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                game_input_event: None,
            },
            ExpectedParams {
                game_input_event: None,
            },
        )
    }

    #[test]
    fn inserts_resources_on_session_accepted() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::HostEstablished,
                game_input_event: Some(GameInputEvent::ActionPressed(PlayerActionControl::new(
                    0,
                    ControlAction::Attack,
                ))),
            },
            ExpectedParams {
                game_input_event: Some(GameInputEvent::ActionPressed(PlayerActionControl::new(
                    0,
                    ControlAction::Attack,
                ))),
            },
        )
    }

    fn run_test(
        SetupParams {
            session_status: session_status_setup,
            game_input_event,
        }: SetupParams,
        ExpectedParams {
            game_input_event: game_input_event_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(<Read<'_, EventChannel<GameInputEvent>> as SystemData>::setup)
            .with_setup(setup_game_input_event_reader)
            .with_system_desc(NetworkInputResponseSystemDesc::default(), "", &[])
            .with_resource(session_status_setup)
            .with_effect(move |world| {
                if let Some(game_input_event) = game_input_event {
                    let socket_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 1234));
                    world
                        .write_resource::<NetEventChannel<GameInputEvent>>()
                        .single_write(NetData {
                            socket_addr,
                            data: game_input_event,
                        });
                }
            })
            .with_assertion(move |world| {
                let (mut game_input_event_rid, network_input_ec) = world.system_data::<(
                    WriteExpect<'_, ReaderId<GameInputEvent>>,
                    Read<'_, EventChannel<GameInputEvent>>,
                )>();
                let game_input_event = network_input_ec.read(&mut *game_input_event_rid).next();

                assert_eq!(game_input_event_expected.as_ref(), game_input_event);
            })
            .run()
    }

    fn setup_game_input_event_reader(world: &mut World) {
        let game_input_event_rid = world
            .write_resource::<EventChannel<GameInputEvent>>()
            .register_reader();
        world.insert(game_input_event_rid);
    }

    struct SetupParams {
        session_status: SessionStatus,
        game_input_event: Option<GameInputEvent>,
    }

    struct ExpectedParams {
        game_input_event: Option<GameInputEvent>,
    }
}
