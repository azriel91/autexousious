#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entities, Entity, ReadStorage, WorldExt},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use game_input_model::{
        config::{Axis, ControlAction, ControlArgs, ControlInputEventArgs},
        play::{AxisMoveEventData, ControlActionEventData, ControlInputEvent, InputControlled},
    };
    use stdio_spi::StdinMapper;

    use game_input_stdio::{ControlInputEventStdinMapper, GameInputStdioError};

    #[test]
    fn maps_axis_input() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_effect(|world| {
                world.register::<InputControlled>();

                let entity = world.create_entity().with(InputControlled::new(1)).build();

                world.insert(entity);
            })
            .with_assertion(|world| {
                let args = ControlInputEventArgs {
                    controller_id: 1,
                    control: ControlArgs::Axis {
                        axis: Axis::X,
                        value: -1.,
                    },
                };
                let result = ControlInputEventStdinMapper::map(
                    &world.system_data::<(Entities, ReadStorage<InputControlled>)>(),
                    args,
                );

                assert!(result.is_ok());
                let entity = *world.read_resource::<Entity>();
                assert_eq!(
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        controller_id: 1,
                        entity,
                        axis: Axis::X,
                        value: -1.,
                    }),
                    result.unwrap()
                )
            })
            .run_isolated()
    }

    #[test]
    fn maps_action_pressed() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_effect(|world| {
                world.register::<InputControlled>();

                let entity = world.create_entity().with(InputControlled::new(1)).build();

                world.insert(entity);
            })
            .with_assertion(|world| {
                let args = ControlInputEventArgs {
                    controller_id: 1,
                    control: ControlArgs::ActionPressed {
                        action: ControlAction::Jump,
                    },
                };
                let result = ControlInputEventStdinMapper::map(
                    &world.system_data::<(Entities, ReadStorage<InputControlled>)>(),
                    args,
                );

                assert!(result.is_ok());
                let entity = *world.read_resource::<Entity>();
                assert_eq!(
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        controller_id: 1,
                        entity,
                        control_action: ControlAction::Jump,
                    }),
                    result.unwrap()
                )
            })
            .run_isolated()
    }

    #[test]
    fn maps_action_released() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_effect(|world| {
                world.register::<InputControlled>();

                let entity = world.create_entity().with(InputControlled::new(1)).build();

                world.insert(entity);
            })
            .with_assertion(|world| {
                let args = ControlInputEventArgs {
                    controller_id: 1,
                    control: ControlArgs::ActionReleased {
                        action: ControlAction::Jump,
                    },
                };
                let result = ControlInputEventStdinMapper::map(
                    &world.system_data::<(Entities, ReadStorage<InputControlled>)>(),
                    args,
                );

                assert!(result.is_ok());
                let entity = *world.read_resource::<Entity>();
                assert_eq!(
                    ControlInputEvent::ControlActionRelease(ControlActionEventData {
                        controller_id: 1,
                        entity,
                        control_action: ControlAction::Jump,
                    }),
                    result.unwrap()
                )
            })
            .run_isolated()
    }

    #[test]
    fn returns_err_when_no_entity_for_controller_id() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_effect(|world| {
                world.register::<InputControlled>();

                world.create_entity().with(InputControlled::new(1)).build();
            })
            .with_assertion(|world| {
                let args = ControlInputEventArgs {
                    controller_id: 2,
                    control: ControlArgs::ActionPressed {
                        action: ControlAction::Jump,
                    },
                };
                let result = ControlInputEventStdinMapper::map(
                    &world.system_data::<(Entities, ReadStorage<InputControlled>)>(),
                    args,
                );

                assert!(result.is_err());
                let error = result.unwrap_err();
                if let Some(game_input_stdio_error) =
                    error.as_error().downcast_ref::<Box<GameInputStdioError>>()
                {
                    let expected_error = GameInputStdioError::EntityWithControllerIdNotFound {
                        controller_id: 2,
                        existent_controllers: vec![1],
                    };
                    assert_eq!(&Box::new(expected_error), game_input_stdio_error);
                } else {
                    // kcov-ignore-start
                    panic!(
                        "Expected `GameInputStdioError` error but was `{:?}`",
                        error.as_error()
                    );
                    // kcov-ignore-end
                }
            })
            .run_isolated()
    }
}
