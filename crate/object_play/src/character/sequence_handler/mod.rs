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
    fn update(
        _character_input: &CharacterInput,
        _character_status: &CharacterStatus,
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
        // No update to run counter.
        let run_counter = None;
        // No calculated next sequence.
        let sequence_id = None;
        // No update to sequence state.
        let sequence_state = None;
        // Don't change facing direction.
        let mirrored = None;
        assert_eq!(
            CharacterStatusUpdate::new(
                run_counter,
                ObjectStatusUpdate::new(sequence_id, sequence_state, mirrored)
            ),
            Sit::update(&CharacterInput::default(), &CharacterStatus::default())
        );
    }

    struct Sit;
    impl SequenceHandler for Sit {}
}
