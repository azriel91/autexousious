use amethyst::{
    animation::AnimationControlSet, assets::AssetStorage, ecs::prelude::*, input::InputHandler,
    renderer::Material,
};
use character_selection::CharacterEntityControl;
use game_input::{Axis, ControlAction, PlayerActionControl, PlayerAxisControl};
use object_model::{
    config::object::character::SequenceId,
    entity::{CharacterInput, ObjectStatus},
    loaded::{Character, CharacterHandle},
};
use object_play::CharacterSequenceHandler;

/// Updates `Character` sequence based on input
#[derive(Debug, Default, new)]
pub(crate) struct CharacterInputUpdateSystem;

type CharacterInputUpdateSystemData<'s, 'c> = (
    Entities<'s>,
    ReadStorage<'s, CharacterHandle>,
    ReadStorage<'s, CharacterEntityControl>,
    Read<'s, AssetStorage<Character>>,
    Read<'s, InputHandler<PlayerAxisControl, PlayerActionControl>>,
    WriteStorage<'s, ObjectStatus<SequenceId>>,
    WriteStorage<'s, AnimationControlSet<SequenceId, Material>>,
);

impl<'s> System<'s> for CharacterInputUpdateSystem {
    type SystemData = CharacterInputUpdateSystemData<'s, 's>;

    fn run(
        &mut self,
        (
            entities,
            handle_storage,
            control_storage,
            characters,
            control_input,
            mut status_storage,
            mut animation_control_set_storage,
        ): Self::SystemData,
    ) {
        for (entity, character_handle, character_entity_control, mut status) in (
            &*entities,
            &handle_storage,
            &control_storage,
            &mut status_storage,
        ).join()
        {
            let player = character_entity_control.controller_id;

            let character = characters
                .get(character_handle)
                .expect("Expected character to be loaded.");

            let x_axis_value = control_input.axis_value(&PlayerAxisControl::new(player, Axis::X));
            let z_axis_value = control_input.axis_value(&PlayerAxisControl::new(player, Axis::Z));

            let input = CharacterInput::new(
                x_axis_value.unwrap_or(0.),
                z_axis_value.unwrap_or(0.),
                control_input
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Defend))
                    .unwrap_or(false),
                control_input
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Jump))
                    .unwrap_or(false),
                control_input
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Attack))
                    .unwrap_or(false),
                control_input
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Special))
                    .unwrap_or(false),
            );
            let next_sequence = CharacterSequenceHandler::update(
                &entity,
                &mut animation_control_set_storage,
                &input,
                character,
                &status.sequence_id,
            );

            // Update the current sequence ID
            if let Some(next_sequence) = next_sequence {
                status.sequence_id = next_sequence;
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}
