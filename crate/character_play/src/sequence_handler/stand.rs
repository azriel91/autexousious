use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{
        common::{
            grounding::AirborneCheck,
            input::{StandXMovementCheck, StandZMovementCheck},
            status::AliveCheck,
            SequenceRepeat,
        },
        CharacterSequenceHandler,
    },
    CharacterSequenceUpdateComponents,
};

/// `Stand` sequence update.
#[derive(Debug)]
pub struct Stand;

impl CharacterSequenceHandler for Stand {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        use character_model::play::RunCounter::*;
        match components.run_counter {
            Exceeded | Increase(_) => panic!(
                "Invalid run_counter state during `Stand` sequence: `{:?}`",
                components.run_counter
            ),
            _ => {}
        };

        [
            AliveCheck::update,
            AirborneCheck::update,
            StandXMovementCheck::update,
            StandZMovementCheck::update,
            SequenceRepeat::update,
        ]
        .iter()
        .fold(None, |status_update, fn_update| {
            status_update.or_else(|| fn_update(components))
        })
    }
}
