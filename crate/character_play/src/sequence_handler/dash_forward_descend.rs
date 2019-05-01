use character_model::config::CharacterSequenceId;

use crate::{
    sequence_handler::{common::SequenceRepeat, CharacterSequenceHandler, SwitchSequenceOnLand},
    CharacterSequenceUpdateComponents,
};

const DASH_FORWARD_DESCEND: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceId::DashDescendLand);

#[derive(Debug)]
pub(crate) struct DashForwardDescend;

impl CharacterSequenceHandler for DashForwardDescend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        DASH_FORWARD_DESCEND
            .update(components)
            .or_else(|| SequenceRepeat::update(components))
    }
}
