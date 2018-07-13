use amethyst::{assets::AssetStorage, ecs::prelude::*};
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, Kinematics},
    loaded::{Character, CharacterHandle},
};

/// Updates `Character` kinematics based on sequence.
#[derive(Debug, Default, new)]
pub(crate) struct CharacterKinematicsSystem;

type CharacterKinematicsSystemData<'s, 'c> = (
    Read<'s, AssetStorage<Character>>,
    ReadStorage<'s, CharacterHandle>,
    ReadStorage<'s, CharacterInput>,
    ReadStorage<'s, CharacterStatus>,
    WriteStorage<'s, Kinematics<f32>>,
);

impl<'s> System<'s> for CharacterKinematicsSystem {
    type SystemData = CharacterKinematicsSystemData<'s, 's>;

    fn run(
        &mut self,
        (
            characters,
            handle_storage,
            character_input_storage,
            status_storage,
            mut kinematics_storage,
        ): Self::SystemData,
    ) {
        for (character_handle, character_input, status, mut kinematics) in (
            &handle_storage,
            &character_input_storage,
            &status_storage,
            &mut kinematics_storage,
        ).join()
        {
            // TODO: Character stats should be configuration.
            // Use the stats from the character definition.
            let _character = characters
                .get(character_handle)
                .expect("Expected character to be loaded.");

            match status.object_status.sequence_id {
                CharacterSequenceId::Stand => {
                    kinematics.velocity[0] = 0.;
                    kinematics.velocity[2] = 0.;
                }
                CharacterSequenceId::Walk => {
                    kinematics.velocity[0] = character_input.x_axis_value as f32 * 3.5;
                    kinematics.velocity[2] = character_input.z_axis_value as f32 * -2.;
                }
                CharacterSequenceId::Run => {
                    kinematics.velocity[0] = character_input.x_axis_value as f32 * 6.;
                    kinematics.velocity[2] = character_input.z_axis_value as f32 * -1.5;
                }
                CharacterSequenceId::StopRun => {
                    kinematics.velocity[0] = if status.object_status.mirrored {
                        -2.
                    } else {
                        2.
                    };
                    kinematics.velocity[2] = character_input.z_axis_value as f32 * -0.5;
                }
                CharacterSequenceId::Jump => {}
                CharacterSequenceId::JumpAscend => {
                    if status.object_status.sequence_state == SequenceState::Begin {
                        kinematics.velocity[1] = 17.;
                    } else {
                        kinematics.velocity[1] += -1.7;
                    }
                }
                CharacterSequenceId::Airborne => {
                    kinematics.velocity[1] += -1.7;
                }
                CharacterSequenceId::AirborneLand => {
                    kinematics.velocity[1] = 0.;
                }
            };
        }
    }
}
