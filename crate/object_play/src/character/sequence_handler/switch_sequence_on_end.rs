use character_model::config::CharacterSequenceId;
use object_model::entity::SequenceStatus;

#[derive(Debug)]
pub(crate) struct SwitchSequenceOnEnd(
    /// The sequence to switch to.
    pub CharacterSequenceId,
);

impl SwitchSequenceOnEnd {
    pub fn update(&self, sequence_status: SequenceStatus) -> Option<CharacterSequenceId> {
        if sequence_status == SequenceStatus::End {
            Some(self.0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use character_model::config::CharacterSequenceId;
    use object_model::entity::SequenceStatus;

    use super::SwitchSequenceOnEnd;

    #[test]
    fn no_update_when_sequence_not_ended() {
        assert_eq!(
            None,
            SwitchSequenceOnEnd(CharacterSequenceId::Stand).update(SequenceStatus::default())
        );
    }

    #[test]
    fn reverts_to_stand_when_sequence_ended() {
        assert_eq!(
            Some(CharacterSequenceId::Stand),
            SwitchSequenceOnEnd(CharacterSequenceId::Stand).update(SequenceStatus::End)
        );
    }
}
