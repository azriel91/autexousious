use object_model::entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate};

pub(super) use self::airborne::Airborne;
pub(super) use self::airborne_land::AirborneLand;
pub(super) use self::jump::Jump;
pub(super) use self::run::Run;
pub(super) use self::stand::Stand;
pub(super) use self::stop_run::StopRun;
pub(super) use self::walk::Walk;

mod airborne;
mod airborne_land;
mod jump;
mod run;
mod stand;
mod stop_run;
mod walk;

/// Traits that every sequence should define for its transition behaviour.
pub(super) trait SequenceHandler {
    /// Updates behaviour in response to input.
    ///
    /// # Parameters
    ///
    /// * `character_input`: Controller input for the character.
    /// * `character_status`: Character specific status attributes.
    /// * `sequence_ended`: Whether the current sequence has ended.
    fn update(
        _character_input: &CharacterInput,
        _character_status: &CharacterStatus,
        _sequence_ended: bool,
    ) -> CharacterStatusUpdate {
        CharacterStatusUpdate::default()
    }
}

#[cfg(test)]
mod test {
    use object_model::entity::{
        CharacterInput, CharacterStatus, CharacterStatusUpdate, ObjectStatusUpdate,
    };

    use super::SequenceHandler;

    #[test]
    fn default_update_is_empty() {
        // Should be `RunCounter::Unused`.
        let run_counter = None;
        // Don't change facing direction.
        let mirrored = None;
        // Use configured next sequence.
        let sequence_id = None;
        assert_eq!(
            CharacterStatusUpdate::new(run_counter, ObjectStatusUpdate::new(sequence_id, mirrored)),
            Sit::update(
                &CharacterInput::default(),
                &CharacterStatus::default(),
                false
            )
        );
    }

    struct Sit;
    impl SequenceHandler for Sit {}
}
