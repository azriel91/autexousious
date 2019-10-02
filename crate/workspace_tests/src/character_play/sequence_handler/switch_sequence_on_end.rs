#[cfg(test)]
mod test {
    use character_model::config::CharacterSequenceName;
    use sequence_model::play::SequenceStatus;

    use character_play::sequence_handler::SwitchSequenceOnEnd;

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
