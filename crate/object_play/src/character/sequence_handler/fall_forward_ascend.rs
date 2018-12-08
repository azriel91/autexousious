use object_model::config::object::CharacterSequenceId;

use crate::{
    character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnDescend},
    CharacterSequenceUpdateComponents,
};

const FALL_FORWARD_ASCEND: SwitchSequenceOnDescend =
    SwitchSequenceOnDescend(CharacterSequenceId::FallForwardDescend);

#[derive(Debug)]
pub(crate) struct FallForwardAscend;

impl CharacterSequenceHandler for FallForwardAscend {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        FALL_FORWARD_ASCEND.update(components)
    }
}
