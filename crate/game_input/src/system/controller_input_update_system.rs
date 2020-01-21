use amethyst::{
    ecs::{Component, Entity, Read, System, SystemData, World, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use derive_new::new;
use game_input_model::{
    Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
};

use crate::ControllerInput;

/// Updates `ControllerInput` based on input events.
#[derive(Debug, Default, new)]
pub struct ControllerInputUpdateSystem {
    /// Reader ID for the `ControlInputEvent` event channel.
    #[new(default)]
    input_events_id: Option<ReaderId<ControlInputEvent>>,
}

type ControllerInputUpdateSystemData<'s> = (
    Read<'s, EventChannel<ControlInputEvent>>,
    WriteStorage<'s, ControllerInput>,
);

impl ControllerInputUpdateSystem {
    fn get_or_insert_mut<'s, C>(comp_storage: &'s mut WriteStorage<C>, entity: Entity) -> &'s mut C
    where
        C: Component + Default,
    {
        if let Ok(entry) = comp_storage.entry(entity) {
            entry.or_insert(C::default());
        }
        comp_storage
            .get_mut(entity)
            .expect("Unreachable: Component either previously existed, or was just inserted.")
    }
}

impl<'s> System<'s> for ControllerInputUpdateSystem {
    type SystemData = ControllerInputUpdateSystemData<'s>;

    fn run(&mut self, (input_events, mut controller_inputs): Self::SystemData) {
        let input_events_id = self
            .input_events_id
            .as_mut()
            .expect("Expected `input_events_id` field to be set.");

        input_events.read(input_events_id).for_each(|ev| match ev {
            ControlInputEvent::AxisMoved(AxisMoveEventData {
                controller_id: _,
                entity,
                axis,
                value,
            }) => {
                let controller_input = Self::get_or_insert_mut(&mut controller_inputs, *entity);
                match axis {
                    Axis::X => controller_input.x_axis_value = *value,
                    Axis::Z => controller_input.z_axis_value = *value,
                };
            }
            ControlInputEvent::ControlActionPress(ControlActionEventData {
                controller_id: _,
                entity,
                control_action,
            }) => {
                let controller_input = Self::get_or_insert_mut(&mut controller_inputs, *entity);
                match control_action {
                    ControlAction::Defend => controller_input.defend = true,
                    ControlAction::Jump => controller_input.jump = true,
                    ControlAction::Attack => controller_input.attack = true,
                    ControlAction::Special => controller_input.special = true,
                };
            }
            ControlInputEvent::ControlActionRelease(ControlActionEventData {
                controller_id: _,
                entity,
                control_action,
            }) => {
                let controller_input = Self::get_or_insert_mut(&mut controller_inputs, *entity);
                match control_action {
                    ControlAction::Defend => controller_input.defend = false,
                    ControlAction::Jump => controller_input.jump = false,
                    ControlAction::Attack => controller_input.attack = false,
                    ControlAction::Special => controller_input.special = false,
                };
            }
        });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.input_events_id = Some(
            world
                .fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}
