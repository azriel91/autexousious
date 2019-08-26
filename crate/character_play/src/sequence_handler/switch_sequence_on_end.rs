use character_model::config::CharacterSequenceName;
use sequence_model::play::SequenceStatus;

#[derive(Debug)]
pub(crate) struct SwitchSequenceOnEnd(
    /// The sequence to switch to.
    pub CharacterSequenceName,
);

impl SwitchSequenceOnEnd {
    pub fn update(&self, sequence_status: SequenceStatus) -> Option<CharacterSequenceName> {
        if sequence_status == SequenceStatus::End {
            Some(self.0.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use character_model::config::CharacterSequenceName;
    use sequence_model::play::SequenceStatus;

    use super::SwitchSequenceOnEnd;

    #[test]
    fn no_update_when_sequence_not_ended() {
        assert_eq!(
            None,
            SwitchSequenceOnEnd(CharacterSequenceName::Stand).update(SequenceStatus::default())
        );
    }

    #[test]
    fn reverts_to_stand_when_sequence_ended() {
        assert_eq!(
            Some(CharacterSequenceName::Stand),
            SwitchSequenceOnEnd(CharacterSequenceName::Stand).update(SequenceStatus::End)
        );
    }
}
