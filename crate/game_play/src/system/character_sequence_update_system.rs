use amethyst::{
    animation::{get_animation_set, AnimationControlSet, ControlState},
    assets::AssetStorage,
    ecs::prelude::*,
    renderer::{Material, MeshHandle},
};
use game_play_state::AnimationRunner;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterInput, CharacterStatus},
    loaded::{Character, CharacterHandle},
};
use object_play::CharacterSequenceHandler;

/// Updates `Character` sequence based on input
#[derive(Debug, Default, new)]
pub(crate) struct CharacterSequenceUpdateSystem;

type CharacterSequenceUpdateSystemData<'s, 'c> = (
    Entities<'s>,
    Read<'s, AssetStorage<Character>>,
    ReadStorage<'s, CharacterHandle>,
    ReadStorage<'s, CharacterInput>,
    WriteStorage<'s, CharacterStatus>,
    WriteStorage<'s, MeshHandle>,
    WriteStorage<'s, AnimationControlSet<CharacterSequenceId, Material>>,
);

impl<'s> System<'s> for CharacterSequenceUpdateSystem {
    type SystemData = CharacterSequenceUpdateSystemData<'s, 's>;

    fn run(
        &mut self,
        (
            entities,
            characters,
            handle_storage,
            character_input_storage,
            mut character_status_storage,
            mut mesh_handle_storage,
            mut animation_control_set_storage,
        ): Self::SystemData,
    ) {
        for (entity, character_handle, character_input, mut character_status) in (
            &*entities,
            &handle_storage,
            &character_input_storage,
            &mut character_status_storage,
        ).join()
        {
            let character = characters
                .get(character_handle)
                .expect("Expected character to be loaded.");

            let mut animation_set = get_animation_set(&mut animation_control_set_storage, entity);

            // TODO: Calculate a delta from the current status and update
            let sequence_ended = {
                animation_set
                    .animations
                    .iter()
                    .find(|&&(ref id, ref _control)| {
                        id == &character_status.object_status.sequence_id
                    })
                    .map_or(true, |(_id, control)| control.state == ControlState::Done)
            };

            let status_update = CharacterSequenceHandler::update(
                character,
                &character_input,
                &character_status,
                sequence_ended,
            );

            // Update the current sequence ID
            if let Some(next_sequence_id) = status_update.object_status.sequence_id {
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

                AnimationRunner::swap(
                    &mut animation_set,
                    &animation_handle,
                    &character_status.object_status.sequence_id,
                    &next_sequence_id,
                );
            }

            if let Some(mirrored) = status_update.object_status.mirrored {
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

            *character_status += status_update;
        }
    }
}
