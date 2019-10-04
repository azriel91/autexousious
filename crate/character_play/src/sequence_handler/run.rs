use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{
        common::{grounding::AirborneCheck, status::AliveCheck},
        CharacterSequenceHandler,
    },
    CharacterSequenceUpdateComponents,
};

/// Hold forward to run, release to stop running.
#[derive(Debug)]
pub struct Run;

impl CharacterSequenceHandler for Run {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        [AliveCheck::update, AirborneCheck::update]
            .iter()
            .fold(None, |status_update, fn_update| {
                status_update.or_else(|| fn_update(components))
            })
    }
}
