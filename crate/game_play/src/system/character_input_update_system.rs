use amethyst::{ecs::prelude::*, input::InputHandler};
use character_selection::CharacterEntityControl;
use game_input::{Axis, ControlAction, PlayerActionControl, PlayerAxisControl};
use object_model::entity::CharacterInput;

/// Updates `Character` sequence based on input
#[derive(Debug, Default, new)]
pub(crate) struct CharacterInputUpdateSystem;

type CharacterInputUpdateSystemData<'s> = (
    Entities<'s>,
    Read<'s, InputHandler<PlayerAxisControl, PlayerActionControl>>,
    ReadStorage<'s, CharacterEntityControl>,
    WriteStorage<'s, CharacterInput>,
);

impl<'s> System<'s> for CharacterInputUpdateSystem {
    type SystemData = CharacterInputUpdateSystemData<'s>;

    fn run(
        &mut self,
        (entities, input_handler, control_storage, mut character_input_storage): Self::SystemData,
    ) {
        for (entity, character_entity_control) in (&*entities, &control_storage).join() {
            let player = character_entity_control.controller_id;

            let x_axis_value = input_handler.axis_value(&PlayerAxisControl::new(player, Axis::X));
            let z_axis_value = input_handler.axis_value(&PlayerAxisControl::new(player, Axis::Z));

            let input = CharacterInput::new(
                x_axis_value.unwrap_or(0.),
                z_axis_value.unwrap_or(0.),
                input_handler
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Defend))
                    .unwrap_or(false),
                input_handler
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Jump))
                    .unwrap_or(false),
                input_handler
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Attack))
                    .unwrap_or(false),
                input_handler
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Special))
                    .unwrap_or(false),
            );

            character_input_storage
                .insert(entity, input)
                .expect("Failed to replace `CharacterInput` for character.");
        }
    }
}
