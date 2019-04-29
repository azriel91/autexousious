use character_model::config::CharacterSequenceId;

use crate::{
    sequence_handler::{common::SequenceRepeat, CharacterSequenceHandler, SwitchSequenceOnLand},
    CharacterSequenceUpdateComponents,
};

const DASH_BACK_DESCEND: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceId::DashDescendLand);

#[derive(Debug)]
pub(crate) struct DashBackDescend;

impl CharacterSequenceHandler for DashBackDescend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        DASH_BACK_DESCEND
            .update(components)
            .or_else(|| SequenceRepeat::update(components))
    }
}
