#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr};

    use amethyst::{
        ecs::{Read, SystemData, World, WorldExt, WriteExpect},
        input::InputEvent,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_input_model::config::{ControlAction, ControlBindings, PlayerActionControl};
    use net_model::play::{NetData, NetEventChannel};
    use network_session_model::play::SessionStatus;

    use network_input_play::NetworkInputResponseSystemDesc;

    #[test]
    fn does_nothing_when_no_input_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                input_event: None,
            },
            ExpectedParams { input_event: None },
        )
    }

    #[test]
    fn inserts_resources_on_session_accepted() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::HostEstablished,
                input_event: Some(InputEvent::ActionPressed(PlayerActionControl::new(
                    0,
                    ControlAction::Attack,
                ))),
            },
            ExpectedParams {
                input_event: Some(InputEvent::ActionPressed(PlayerActionControl::new(
                    0,
                    ControlAction::Attack,
                ))),
            },
        )
    }

    fn run_test(
        SetupParams {
            session_status: session_status_setup,
            input_event,
        }: SetupParams,
        ExpectedParams {
            input_event: input_event_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(<Read<'_, EventChannel<InputEvent<ControlBindings>>> as SystemData>::setup)
            .with_setup(setup_input_event_reader)
            .with_system_desc(NetworkInputResponseSystemDesc::default(), "", &[])
            .with_resource(session_status_setup)
            .with_effect(move |world| {
                if let Some(input_event) = input_event {
                    let socket_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 1234));
                    world
                        .write_resource::<NetEventChannel<InputEvent<ControlBindings>>>()
                        .single_write(NetData {
                            socket_addr,
                            data: input_event,
                        });
                }
            })
            .with_assertion(move |world| {
                let (mut input_event_rid, network_input_ec) = world.system_data::<(
                    WriteExpect<'_, ReaderId<InputEvent<ControlBindings>>>,
                    Read<'_, EventChannel<InputEvent<ControlBindings>>>,
                )>();
                let input_event = network_input_ec.read(&mut *input_event_rid).next();

                assert_eq!(input_event_expected.as_ref(), input_event);
            })
            .run()
    }

    fn setup_input_event_reader(world: &mut World) {
        let input_event_rid = world
            .write_resource::<EventChannel<InputEvent<ControlBindings>>>()
            .register_reader();
        world.insert(input_event_rid);
    }

    struct SetupParams {
        session_status: SessionStatus,
        input_event: Option<InputEvent<ControlBindings>>,
    }

    struct ExpectedParams {
        input_event: Option<InputEvent<ControlBindings>>,
    }
}
