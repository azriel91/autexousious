use character_model::config::CharacterSequenceId;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnDescend},
    CharacterSequenceUpdateComponents,
};

const DASH_FORWARD_ASCEND: SwitchSequenceOnDescend =
    SwitchSequenceOnDescend(CharacterSequenceId::DashForwardDescend);

#[derive(Debug)]
pub(crate) struct DashForwardAscend;

impl CharacterSequenceHandler for DashForwardAscend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        DASH_FORWARD_ASCEND.update(components)
    }
}
