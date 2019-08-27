use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnDescend},
    CharacterSequenceUpdateComponents,
};

const DASH_BACK_ASCEND: SwitchSequenceOnDescend =
    SwitchSequenceOnDescend(CharacterSequenceName::DashBackDescend);

#[derive(Debug)]
pub(crate) struct DashBackAscend;

impl CharacterSequenceHandler for DashBackAscend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        DASH_BACK_ASCEND.update(components)
    }
}
