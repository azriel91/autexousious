use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{
        common::{grounding::AirborneCheck, status::AliveCheck},
        CharacterSequenceHandler, SwitchSequenceOnEnd,
    },
    CharacterSequenceUpdateComponents,
};

const DODGE: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceName::Stand);

/// `Dodge` sequence update.
#[derive(Debug)]
pub struct Dodge;

impl CharacterSequenceHandler for Dodge {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        [AliveCheck::update, AirborneCheck::update]
            .iter()
            .fold(None, |status_update, fn_update| {
                status_update.or_else(|| fn_update(components))
            })
            .or_else(|| DODGE.update(components.sequence_status))
    }
}
