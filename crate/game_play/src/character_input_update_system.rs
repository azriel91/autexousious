use amethyst::{
    animation::{get_animation_set, AnimationControlSet},
    assets::AssetStorage,
    ecs::prelude::*,
    input::InputHandler,
    renderer::{Material, MeshHandle},
};
use character_selection::CharacterEntityControl;
use game_input::{Axis, ControlAction, PlayerActionControl, PlayerAxisControl};
use object_model::{
    config::object::character::SequenceId,
    entity::{CharacterInput, ObjectStatus},
    loaded::{Character, CharacterHandle},
};
use object_play::CharacterSequenceHandler;

use AnimationRunner;

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
    WriteStorage<'s, MeshHandle>,
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
            mut mesh_handle_storage,
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

            // TODO: Calculate a delta from the current status and update
            let status_update =
                CharacterSequenceHandler::update(character, &input, &status.sequence_id);

            // Update the current sequence ID
            if let Some(next_sequence_id) = status_update.sequence_id {
                let animation_handle = &character
                    .object
                    .animations
                    .get(&next_sequence_id)
                    .unwrap_or_else(|| {
                        panic!(
                            "Failed to get animation for sequence: `{:?}`",
                            next_sequence_id
                        )
                    })
                    .clone();

                let mut animation_set =
                    get_animation_set(&mut animation_control_set_storage, entity);

                AnimationRunner::swap(
                    &mut animation_set,
                    &animation_handle,
                    &status.sequence_id,
                    &next_sequence_id,
                );
            }

            if let Some(mirrored) = status_update.mirrored {
                // Swap the current mesh with the appropriate mesh.
                let mesh_handle = if mirrored {
                    character.object.mesh_mirrored.clone()
                } else {
                    character.object.mesh.clone()
                };
                mesh_handle_storage
                    .insert(entity, mesh_handle)
                    .expect("Failed to replace mesh for character.");
            }

            *status += status_update;
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}
