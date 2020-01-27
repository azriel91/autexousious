use amethyst::{
    ecs::{Entities, Join, ReadStorage},
    Error,
};
use game_input::InputControlled;
use game_input_model::{
    AxisMoveEventData, ControlActionEventData, ControlArgs, ControlInputEvent,
    ControlInputEventArgs,
};
use stdio_spi::{MapperSystemData, StdinMapper};

use crate::GameInputStdioError;

#[derive(Debug)]
pub struct ControlInputEventStdinMapperData;

impl<'s> MapperSystemData<'s> for ControlInputEventStdinMapperData {
    type SystemData = (Entities<'s>, ReadStorage<'s, InputControlled>);
}

/// Builds a `ControlInputEvent` from stdin tokens.
#[derive(Debug)]
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
            controller_id,
            control,
        } = args;

        (entities, input_controlleds)
            .join()
            .find(|(_e, input_controlled)| input_controlled.controller_id == controller_id)
            .map(|(entity, _input_controlled)| match control {
                ControlArgs::Axis { axis, value } => {
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        controller_id,
                        entity,
                        axis,
                        value,
                    })
                }
                ControlArgs::ActionPressed { action } => {
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        controller_id,
                        entity,
                        control_action: action,
                    })
                }
                ControlArgs::ActionReleased { action } => {
                    ControlInputEvent::ControlActionRelease(ControlActionEventData {
                        controller_id,
                        entity,
                        control_action: action,
                    })
                }
            })
            .ok_or_else(|| {
                let existent_controllers = input_controlleds
                    .join()
                    .map(|input_controlled| input_controlled.controller_id)
                    .collect::<Vec<_>>();
                Error::new(GameInputStdioError::EntityWithControllerIdNotFound {
                    controller_id,
                    existent_controllers,
                })
            })
    }
}
