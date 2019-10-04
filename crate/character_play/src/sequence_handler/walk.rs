use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{
        common::{
            grounding::AirborneCheck,
            input::{WalkXMovementCheck, WalkZMovementCheck},
            status::AliveCheck,
        },
        CharacterSequenceHandler,
    },
    CharacterSequenceUpdateComponents,
};

/// `Walk` sequence update.
#[derive(Debug)]
pub struct Walk;

impl CharacterSequenceHandler for Walk {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        [
            AliveCheck::update,
            AirborneCheck::update,
            WalkXMovementCheck::update,
            WalkZMovementCheck::update,
        ]
        .iter()
        .fold(None, |status_update, fn_update| {
            status_update.or_else(|| fn_update(components))
        })
    }
}
