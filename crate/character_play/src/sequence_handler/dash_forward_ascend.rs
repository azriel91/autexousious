use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnDescend},
    CharacterSequenceUpdateComponents,
};

const DASH_FORWARD_ASCEND: SwitchSequenceOnDescend =
    SwitchSequenceOnDescend(CharacterSequenceName::DashForwardDescend);

#[derive(Debug)]
pub(crate) struct DashForwardAscend;

impl CharacterSequenceHandler for DashForwardAscend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        DASH_FORWARD_ASCEND.update(components)
    }
}
