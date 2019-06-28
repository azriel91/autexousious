use character_model::config::CharacterSequenceId;
use sequence_model::{
    config::SequenceEndTransition, loaded::SequenceEndTransitions, play::SequenceStatus,
};

use crate::{
    sequence_handler::{
        CharacterSequenceHandler, DashAttack, DashBack, DashBackAscend, DashBackDescend,
        DashDescendLand, DashForward, DashForwardAscend, DashForwardDescend, Dodge,
        FallForwardAscend, FallForwardDescend, FallForwardLand, Jump, JumpAscend, JumpAttack,
        JumpDescend, JumpDescendLand, JumpOff, LieFaceDown, Run, RunStop, Stand, StandAttack,
        StandOnSequenceEnd, Walk,
    },
    CharacterSequenceUpdateComponents,
};

/// Defines behaviour for a character in game.
#[derive(Debug)]
pub struct CharacterSequenceUpdater;

impl CharacterSequenceUpdater {
    /// Handles behaviour transition (if any) based on components.controller_input.
    ///
    /// # Parameters
    ///
    /// * `components`: Components used to compute character sequence updates.
    pub fn update<'c>(
        sequence_end_transitions: &SequenceEndTransitions<CharacterSequenceId>,
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        let sequence_handler: &dyn Fn(
            CharacterSequenceUpdateComponents<'_>,
        ) -> Option<CharacterSequenceId> = match components.character_sequence_id {
            CharacterSequenceId::Stand => &Stand::update,
            CharacterSequenceId::StandAttack0 | CharacterSequenceId::StandAttack1 => {
                &StandAttack::update
            }
            CharacterSequenceId::Walk => &Walk::update,
            CharacterSequenceId::Run => &Run::update,
            CharacterSequenceId::RunStop => &RunStop::update,
            CharacterSequenceId::Dodge => &Dodge::update,
            CharacterSequenceId::Jump => &Jump::update,
            CharacterSequenceId::JumpOff => &JumpOff::update,
            CharacterSequenceId::JumpAscend => &JumpAscend::update,
            CharacterSequenceId::JumpDescend => &JumpDescend::update,
            CharacterSequenceId::JumpDescendLand => &JumpDescendLand::update,
            CharacterSequenceId::JumpAttack => &JumpAttack::update,
            CharacterSequenceId::Flinch0
            | CharacterSequenceId::Flinch1
            | CharacterSequenceId::Dazed => &StandOnSequenceEnd::update,
            CharacterSequenceId::FallForwardAscend => &FallForwardAscend::update,
            CharacterSequenceId::FallForwardDescend => &FallForwardDescend::update,
            CharacterSequenceId::FallForwardLand => &FallForwardLand::update,
            CharacterSequenceId::LieFaceDown => &LieFaceDown::update,
            CharacterSequenceId::DashForward => &DashForward::update,
            CharacterSequenceId::DashForwardAscend => &DashForwardAscend::update,
            CharacterSequenceId::DashForwardDescend => &DashForwardDescend::update,
            CharacterSequenceId::DashBack => &DashBack::update,
            CharacterSequenceId::DashBackAscend => &DashBackAscend::update,
            CharacterSequenceId::DashBackDescend => &DashBackDescend::update,
            CharacterSequenceId::DashDescendLand => &DashDescendLand::update,
            CharacterSequenceId::DashAttack => &DashAttack::update,
        };

        let mut sequence_id = sequence_handler(components);

        // Check if it's at the end of the sequence before switching to next.
        if components.sequence_status == SequenceStatus::End {
            let current_sequence_id = &components.character_sequence_id;
            let sequence_end_transition =
                sequence_end_transitions.get(current_sequence_id).copied();

            // `next` from configuration overrides the state handler transition.
            if let Some(sequence_end_transition) = sequence_end_transition {
                match sequence_end_transition {
                    SequenceEndTransition::None | SequenceEndTransition::Delete => {}
                    SequenceEndTransition::Repeat => {
                        sequence_id = Some(components.character_sequence_id);
                    }
                    SequenceEndTransition::SequenceId(next_sequence_id) => {
                        sequence_id = Some(next_sequence_id);
                    }
                }
            }
        }

        sequence_id

        // TODO: overrides based on sequence configuration
    }
}
