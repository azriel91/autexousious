use amethyst::{
    ecs::{Entities, Join, ReadStorage, WorldExt},
    Error,
};
use game_input::InputControlled;
use game_input_model::{AxisMoveEventData, ControlActionEventData, ControlInputEvent};
use stdio_spi::{MapperSystemData, StdinMapper};
use typename_derive::TypeName;

use crate::{ControlArgs, ControlInputEventArgs, GameInputStdioError};

#[derive(Debug)]
pub struct ControlInputEventStdinMapperData;

impl<'s> MapperSystemData<'s> for ControlInputEventStdinMapperData {
    type SystemData = (Entities<'s>, ReadStorage<'s, InputControlled>);
}

/// Builds a `ControlInputEvent` from stdin tokens.
#[derive(Debug, TypeName)]
pub struct ControlInputEventStdinMapper;

impl StdinMapper for ControlInputEventStdinMapper {
    type SystemData = ControlInputEventStdinMapperData;
    type Event = ControlInputEvent;
    type Args = ControlInputEventArgs;

    fn map(
        (entities, input_controlleds): &<Self::SystemData as MapperSystemData>::SystemData,
        args: Self::Args,
    ) -> Result<Self::Event, Error> {
        let ControlInputEventArgs {
            controller,
            control,
        } = &args;

        (entities, input_controlleds)
            .join()
            .find(|(_e, input_controlled)| input_controlled.controller_id == *controller)
            .map(|(entity, _input_controlled)| match control {
                ControlArgs::Axis { axis, value } => {
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        entity,
                        axis: *axis,
                        value: *value,
                    })
                }
                ControlArgs::ActionPressed { action } => {
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        entity,
                        control_action: *action,
                    })
                }
                ControlArgs::ActionReleased { action } => {
                    ControlInputEvent::ControlActionRelease(ControlActionEventData {
                        entity,
                        control_action: *action,
                    })
                }
            })
            .ok_or_else(|| {
                let existent_controllers = input_controlleds
                    .join()
                    .map(|input_controlled| input_controlled.controller_id)
                    .collect::<Vec<_>>();
                Error::new(GameInputStdioError::EntityWithControllerIdNotFound {
                    controller_id: *controller,
                    existent_controllers,
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entities, Entity, ReadStorage, WorldExt},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use game_input::InputControlled;
    use game_input_model::{
        Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
    };
    use stdio_spi::StdinMapper;

    use super::ControlInputEventStdinMapper;
    use crate::{ControlArgs, ControlInputEventArgs, GameInputStdioError};

    #[test]
    fn maps_axis_input() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_setup(|world| {
                world.register::<InputControlled>();

                let entity = world.create_entity().with(InputControlled::new(1)).build();

                world.insert(entity);
            })
            .with_assertion(|world| {
                let args = ControlInputEventArgs {
                    controller: 1,
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
            .with_setup(|world| {
                world.register::<InputControlled>();

                let entity = world.create_entity().with(InputControlled::new(1)).build();

                world.insert(entity);
            })
            .with_assertion(|world| {
                let args = ControlInputEventArgs {
                    controller: 1,
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
            .with_setup(|world| {
                world.register::<InputControlled>();

                let entity = world.create_entity().with(InputControlled::new(1)).build();

                world.insert(entity);
            })
            .with_assertion(|world| {
                let args = ControlInputEventArgs {
                    controller: 1,
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
            .with_setup(|world| {
                world.register::<InputControlled>();

                world.create_entity().with(InputControlled::new(1)).build();
            })
            .with_assertion(|world| {
                let args = ControlInputEventArgs {
                    controller: 2,
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
