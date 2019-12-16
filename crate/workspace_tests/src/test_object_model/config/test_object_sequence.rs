#[cfg(test)]
mod tests {
    use object_model::config::{ObjectFrame, ObjectSequence};
    use sequence_model::config::{Sequence, SequenceEndTransition, Wait};
    use serde_yaml;
    use sprite_model::config::SpriteRef;

    use test_object_model::config::{TestObjectFrame, TestObjectSequence};

    const SEQUENCE_WITH_FRAMES_EMPTY: &str = "frames: []";
    const SEQUENCE_WITH_INPUT_REACTIONS: &str = r#"---
frames:
  - wait: 2
    sprite: { sheet: 0, index: 4 }
"#;

    #[test]
    fn sequence_with_empty_frames_list_deserializes_successfully() {
        let sequence = serde_yaml::from_str::<TestObjectSequence>(SEQUENCE_WITH_FRAMES_EMPTY)
            .expect("Failed to deserialize sequence.");

        let expected = TestObjectSequence::new(ObjectSequence::default());
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_input_reactions() {
        let sequence = serde_yaml::from_str::<TestObjectSequence>(SEQUENCE_WITH_INPUT_REACTIONS)
            .expect("Failed to deserialize sequence.");

        let frames = vec![TestObjectFrame::new(ObjectFrame {
            wait: Wait::new(2),
            sprite: SpriteRef::new(0, 4),
            ..Default::default()
        })];
        let expected = TestObjectSequence::new(ObjectSequence {
            sequence: Sequence {
                next: SequenceEndTransition::None,
                frames,
            },
            ..Default::default()
        });

        assert_eq!(expected, sequence);
    }
}
