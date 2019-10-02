use character_model::config::CharacterSequenceName;
use sequence_model::play::SequenceStatus;

use crate::{
    sequence_handler::{
        common::{grounding::AirborneCheck, status::AliveCheck},
        CharacterSequenceHandler,
    },
    CharacterSequenceUpdateComponents,
};

/// `RunStop` sequence update.
#[derive(Debug)]
pub struct RunStop;

impl CharacterSequenceHandler for RunStop {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        [AliveCheck::update, AirborneCheck::update]
            .iter()
            .fold(None, |status_update, fn_update| {
                status_update.or_else(|| fn_update(components))
            })
            .or_else(|| {
                if components.sequence_status == SequenceStatus::End {
                    Some(CharacterSequenceName::Stand)
                } else {
                    None
                }
            })
    }
}
