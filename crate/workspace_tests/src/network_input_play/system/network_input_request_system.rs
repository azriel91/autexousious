#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use amethyst::{
        ecs::{Read, SystemData, World, WorldExt, WriteExpect},
        input::InputEvent,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_input_model::{
        config::{ControlAction, ControlBindings, PlayerActionControl},
        GameInputEvent,
    };
    use net_model::play::NetMessageEvent;
    use network_session_model::play::SessionStatus;

    use network_input_play::NetworkInputRequestSystemDesc;

    #[test]
    fn does_nothing_when_no_input_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::HostEstablished,
                input_event: None,
            },
            ExpectedParams {
                session_status: SessionStatus::HostEstablished,
                net_message_event: None,
            },
        )
    }

    #[test]
    fn sends_net_message_event_on_input_event_when_host_established() -> Result<(), Error> {
        let input_event =
            InputEvent::ActionPressed(PlayerActionControl::new(0, ControlAction::Attack));
        let game_input_event = GameInputEvent::try_from(input_event.clone())
            .expect("Failed to convert `InputEvent<ControlBindings>` to `GameInputEvent`.");

        run_test(
            SetupParams {
                session_status: SessionStatus::HostEstablished,
                input_event: Some(input_event),
            },
            ExpectedParams {
                session_status: SessionStatus::HostEstablished,
                net_message_event: Some(NetMessageEvent::GameInputEvent(game_input_event)),
            },
        )
    }

    #[test]
    fn sends_net_message_event_on_input_event_when_join_established() -> Result<(), Error> {
        let input_event =
            InputEvent::ActionPressed(PlayerActionControl::new(0, ControlAction::Attack));
        let game_input_event = GameInputEvent::try_from(input_event.clone())
            .expect("Failed to convert `InputEvent<ControlBindings>` to `GameInputEvent`.");

        run_test(
            SetupParams {
                session_status: SessionStatus::JoinEstablished,
                input_event: Some(input_event),
            },
            ExpectedParams {
                session_status: SessionStatus::JoinEstablished,
                net_message_event: Some(NetMessageEvent::GameInputEvent(game_input_event)),
            },
        )
    }

    #[test]
    fn ignores_network_input_request_when_session_not_established() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_status: SessionStatus::None,
                input_event: Some(InputEvent::ActionPressed(PlayerActionControl::new(
                    0,
                    ControlAction::Attack,
                ))),
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
            input_event,
        }: SetupParams,
        ExpectedParams {
            session_status: session_status_expected,
            net_message_event: net_message_event_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(<Read<'_, EventChannel<NetMessageEvent>> as SystemData>::setup)
            .with_setup(setup_net_message_event_reader)
            .with_system_desc(NetworkInputRequestSystemDesc::default(), "", &[])
            .with_resource(session_status_setup)
            .with_effect(move |world| {
                if let Some(input_event) = input_event {
                    world
                        .write_resource::<EventChannel<InputEvent<ControlBindings>>>()
                        .single_write(input_event);
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
        input_event: Option<InputEvent<ControlBindings>>,
    }

    struct ExpectedParams {
        session_status: SessionStatus,
        net_message_event: Option<NetMessageEvent>,
    }
}
