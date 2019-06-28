use character_model::config::CharacterSequenceId;

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

        sequence_handler(components)

        // TODO: overrides based on sequence configuration
    }
}
