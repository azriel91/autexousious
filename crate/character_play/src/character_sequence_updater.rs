use character_model::config::CharacterSequenceName;

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
    ) -> Option<CharacterSequenceName> {
        let sequence_handler: &dyn Fn(
            CharacterSequenceUpdateComponents<'_>,
        ) -> Option<CharacterSequenceName> = match components.character_sequence_name {
            CharacterSequenceName::Stand => &Stand::update,
            CharacterSequenceName::StandAttack0 | CharacterSequenceName::StandAttack1 => {
                &StandAttack::update
            }
            CharacterSequenceName::Walk => &Walk::update,
            CharacterSequenceName::Run => &Run::update,
            CharacterSequenceName::RunStop => &RunStop::update,
            CharacterSequenceName::Dodge => &Dodge::update,
            CharacterSequenceName::Jump => &Jump::update,
            CharacterSequenceName::JumpOff => &JumpOff::update,
            CharacterSequenceName::JumpAscend => &JumpAscend::update,
            CharacterSequenceName::JumpDescend => &JumpDescend::update,
            CharacterSequenceName::JumpDescendLand => &JumpDescendLand::update,
            CharacterSequenceName::JumpAttack => &JumpAttack::update,
            CharacterSequenceName::Flinch0
            | CharacterSequenceName::Flinch1
            | CharacterSequenceName::Dazed => &StandOnSequenceEnd::update,
            CharacterSequenceName::FallForwardAscend => &FallForwardAscend::update,
            CharacterSequenceName::FallForwardDescend => &FallForwardDescend::update,
            CharacterSequenceName::FallForwardLand => &FallForwardLand::update,
            CharacterSequenceName::LieFaceDown => &LieFaceDown::update,
            CharacterSequenceName::DashForward => &DashForward::update,
            CharacterSequenceName::DashForwardAscend => &DashForwardAscend::update,
            CharacterSequenceName::DashForwardDescend => &DashForwardDescend::update,
            CharacterSequenceName::DashBack => &DashBack::update,
            CharacterSequenceName::DashBackAscend => &DashBackAscend::update,
            CharacterSequenceName::DashBackDescend => &DashBackDescend::update,
            CharacterSequenceName::DashDescendLand => &DashDescendLand::update,
            CharacterSequenceName::DashAttack => &DashAttack::update,
        };

        sequence_handler(components)

        // TODO: overrides based on sequence configuration
    }
}
