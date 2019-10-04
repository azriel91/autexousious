use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{
        CharacterSequenceHandler, SwitchSequenceOnEndYVelocity, SwitchSequenceOnLand,
    },
    CharacterSequenceUpdateComponents,
};

/// Disallow dash when landing in the middle of an attack.
const JUMP_ATTACK_NOT_END: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceName::DashDescendLand);

/// Switch to the appropriate sequence based on velocity after attacking.
const JUMP_ATTACK_END: SwitchSequenceOnEndYVelocity = SwitchSequenceOnEndYVelocity {
    upwards: CharacterSequenceName::JumpAscend,
    downwards: CharacterSequenceName::JumpDescend,
};

/// `JumpAttack` sequence update.
#[derive(Debug)]
pub struct JumpAttack;

impl CharacterSequenceHandler for JumpAttack {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        JUMP_ATTACK_NOT_END
            .update(components)
            .or_else(|| JUMP_ATTACK_END.update(components))
    }
}
