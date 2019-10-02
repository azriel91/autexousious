use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{common::SequenceRepeat, CharacterSequenceHandler, SwitchSequenceOnLand},
    CharacterSequenceUpdateComponents,
};

const DASH_BACK_DESCEND: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceName::DashDescendLand);

/// `DashBackDescend` sequence update.
#[derive(Debug)]
pub struct DashBackDescend;

impl CharacterSequenceHandler for DashBackDescend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        DASH_BACK_DESCEND
            .update(components)
            .or_else(|| SequenceRepeat::update(components))
    }
}
