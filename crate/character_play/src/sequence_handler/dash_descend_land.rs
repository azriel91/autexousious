use character_model::config::CharacterSequenceName;

use crate::sequence_handler::{
    switch_sequence_on_end::SwitchSequenceOnEnd, CharacterSequenceHandler,
    CharacterSequenceUpdateComponents,
};

const DASH_DESCEND_LAND: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceName::Stand);

#[derive(Debug)]
pub(crate) struct DashDescendLand;

impl CharacterSequenceHandler for DashDescendLand {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        DASH_DESCEND_LAND.update(components.sequence_status)
    }
}
