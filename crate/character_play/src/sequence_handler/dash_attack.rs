use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{
        CharacterSequenceHandler, SwitchSequenceOnEndYVelocity, SwitchSequenceOnLand,
    },
    CharacterSequenceUpdateComponents,
};

/// Disallow dash when landing in the middle of an attack.
const DASH_ATTACK_NOT_END: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceName::DashDescendLand);

/// Switch to the appropriate sequence based on velocity after attacking.
const DASH_ATTACK_END: SwitchSequenceOnEndYVelocity = SwitchSequenceOnEndYVelocity {
    upwards: CharacterSequenceName::DashForwardAscend,
    downwards: CharacterSequenceName::DashForwardDescend,
};

/// `DashAttack` sequence update.
#[derive(Debug)]
pub struct DashAttack;

impl CharacterSequenceHandler for DashAttack {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        DASH_ATTACK_NOT_END
            .update(components)
            .or_else(|| DASH_ATTACK_END.update(components))
    }
}
