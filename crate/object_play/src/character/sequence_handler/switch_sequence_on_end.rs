use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{ObjectStatus, ObjectStatusUpdate},
};

#[derive(Debug)]
pub(crate) struct SwitchSequenceOnEnd(
    /// The sequence to switch to.
    pub CharacterSequenceId,
);

impl SwitchSequenceOnEnd {
    pub fn update(
        &self,
        object_status: &ObjectStatus<CharacterSequenceId>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        let mut object_status_update = ObjectStatusUpdate::default();
        if object_status.sequence_state == SequenceState::End {
            object_status_update.sequence_id = Some(self.0);
            object_status_update.sequence_state = Some(SequenceState::Begin);
        }

        object_status_update
    }
}

#[cfg(test)]
mod test {
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{ObjectStatus, ObjectStatusUpdate},
    };

    use super::SwitchSequenceOnEnd;

    #[test]
    fn no_update_when_sequence_not_ended() {
        assert_eq!(
            ObjectStatusUpdate::default(),
            SwitchSequenceOnEnd(CharacterSequenceId::Stand).update(&ObjectStatus {
                sequence_id: CharacterSequenceId::Flinch0,
                ..Default::default()
            })
        );
    }

    #[test]
    fn reverts_to_stand_when_sequence_ended() {
        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Stand),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            },
            SwitchSequenceOnEnd(CharacterSequenceId::Stand).update(&ObjectStatus {
                sequence_id: CharacterSequenceId::Flinch0,
                sequence_state: SequenceState::End,
                ..Default::default()
            })
        );
    }
}
