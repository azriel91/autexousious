#[cfg(test)]
mod tests {
    use character_model::config::CharacterSequenceName;
    use game_input_model::play::ControllerInput;
    use mirrored_model::play::Mirrored;
    use sequence_model::config::SequenceNameString;

    use character_play::MirroredUpdater;

    #[test]
    fn no_change_when_no_input() {
        let input = ControllerInput::default();

        vec![true, false].into_iter().for_each(|mirrored| {
            verify_for_sequences(mirrored.into(), &input, mirrored.into());
        });
    }

    #[test]
    fn no_change_when_non_applicable_sequences() {
        let input = ControllerInput::default();

        vec![true, false].into_iter().for_each(|mirrored| {
            verify_for_sequences_na(&input, mirrored.into());
        });
    }

    #[test]
    fn true_when_input_is_left() {
        let mut input = ControllerInput::default();
        input.x_axis_value = -1.;

        vec![true, false].into_iter().for_each(|mirrored| {
            verify_for_sequences(Mirrored(true), &input, mirrored.into());
        });
    }

    #[test]
    fn false_when_input_is_right() {
        let mut input = ControllerInput::default();
        input.x_axis_value = 1.;

        vec![true, false].into_iter().for_each(|mirrored| {
            verify_for_sequences(Mirrored(false), &input, mirrored.into());
        });
    }

    fn verify_for_sequences(
        expected: Mirrored,
        controller_input: &ControllerInput,
        mirrored: Mirrored,
    ) {
        vec![
            CharacterSequenceName::Stand,
            CharacterSequenceName::Walk,
            CharacterSequenceName::JumpAscend,
            CharacterSequenceName::JumpDescend,
        ]
        .into_iter()
        .for_each(|sequence_id| {
            assert_eq!(
                expected,
                MirroredUpdater::update(
                    controller_input,
                    &SequenceNameString::Name(sequence_id),
                    mirrored,
                )
            );
        });
    }

    fn verify_for_sequences_na(controller_input: &ControllerInput, mirrored: Mirrored) {
        vec![CharacterSequenceName::Run, CharacterSequenceName::RunStop]
            .into_iter()
            .for_each(|sequence_id| {
                assert_eq!(
                    mirrored,
                    MirroredUpdater::update(
                        controller_input,
                        &SequenceNameString::Name(sequence_id),
                        mirrored,
                    )
                );
            });
    }
}
