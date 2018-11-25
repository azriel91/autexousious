use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, Kinematics, ObjectStatus, ObjectStatusUpdate, RunCounter},
    loaded::Character,
};

use character::sequence_handler::{
    CharacterSequenceHandler, FallForwardAscend, FallForwardDescend, FallForwardLand, Jump,
    JumpAscend, JumpDescend, JumpDescendLand, JumpOff, LieFaceDown, Run, RunStop, Stand,
    StandAttack, StandOnSequenceEnd, Walk,
};

/// Defines behaviour for a character in game.
#[derive(Debug)]
pub struct CharacterSequenceUpdater;

impl CharacterSequenceUpdater {
    /// Handles behaviour transition (if any) based on input.
    ///
    /// # Parameters
    ///
    /// * `character`: Loaded character configuration.
    /// * `controller_input`: Controller input for the character.
    /// * `character_status`: Character specific status attributes.
    /// * `object_status`: Object status attributes.
    /// * `sequence_ended`: Whether the current sequence has ended.
    /// * `kinematics`: Kinematics of the character.
    pub fn update(
        character: &Character,
        controller_input: &ControllerInput,
        character_status: &CharacterStatus,
        object_status: &ObjectStatus<CharacterSequenceId>,
        kinematics: &Kinematics<f32>,
        run_counter: RunCounter,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        let sequence_handler: &Fn(
            &ControllerInput,
            &CharacterStatus,
            &ObjectStatus<CharacterSequenceId>,
            &Kinematics<f32>,
            RunCounter,
        ) -> ObjectStatusUpdate<CharacterSequenceId> = match object_status.sequence_id {
            CharacterSequenceId::Stand => &Stand::update,
            CharacterSequenceId::StandAttack => &StandAttack::update,
            CharacterSequenceId::Walk => &Walk::update,
            CharacterSequenceId::Run => &Run::update,
            CharacterSequenceId::RunStop => &RunStop::update,
            CharacterSequenceId::Jump => &Jump::update,
            CharacterSequenceId::JumpOff => &JumpOff::update,
            CharacterSequenceId::JumpAscend => &JumpAscend::update,
            CharacterSequenceId::JumpDescend => &JumpDescend::update,
            CharacterSequenceId::JumpDescendLand => &JumpDescendLand::update,
            CharacterSequenceId::Flinch0 | CharacterSequenceId::Flinch1 => {
                &StandOnSequenceEnd::update
            }
            CharacterSequenceId::FallForwardAscend => &FallForwardAscend::update,
            CharacterSequenceId::FallForwardDescend => &FallForwardDescend::update,
            CharacterSequenceId::FallForwardLand => &FallForwardLand::update,
            CharacterSequenceId::LieFaceDown => &LieFaceDown::update,
        };

        let mut object_status_update = sequence_handler(
            controller_input,
            character_status,
            object_status,
            kinematics,
            run_counter,
        );

        // Check if it's at the end of the sequence before switching to next.
        if object_status.sequence_state == SequenceState::End {
            let current_sequence_id = &object_status.sequence_id;
            let current_sequence =
                &character.definition.object_definition.sequences[current_sequence_id];

            // `next` from configuration overrides the state handler transition.
            if current_sequence.next.is_some() {
                object_status_update.sequence_id = current_sequence.next;
            }
        }

        object_status_update

        // TODO: overrides based on sequence configuration
    }
}
