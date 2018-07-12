use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate},
};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct Airborne;

impl SequenceHandler for Airborne {
    fn update(
        _character_input: &CharacterInput,
        _character_status: &CharacterStatus,
        sequence_ended: bool,
    ) -> CharacterStatusUpdate {
        let mut update = CharacterStatusUpdate::default();
        if sequence_ended {
            update.object_status.sequence_id = Some(CharacterSequenceId::Airborne)
        }

        update
    }
}

#[cfg(test)]
mod test {
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterInput, CharacterStatus, CharacterStatusUpdate, ObjectStatus,
            ObjectStatusUpdate, RunCounter,
        },
    };

    use super::Airborne;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(None, ObjectStatusUpdate::new(None, None)),
            Airborne::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Airborne, false)
                ),
                false
            )
        );
    }

    #[test]
    fn restarts_airborne_when_sequence_ends() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                None,
                ObjectStatusUpdate::new(Some(CharacterSequenceId::Airborne), None)
            ),
            Airborne::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Airborne, false)
                ),
                true
            )
        );
    }
}
