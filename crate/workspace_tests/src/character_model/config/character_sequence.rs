#[cfg(test)]
mod tests {
    use object_model::config::{ObjectFrame, ObjectSequence};
    use sequence_model::config::{
        ControlTransition, ControlTransitionSingle, SequenceEndTransition, SequenceNameString, Wait,
    };
    use serde_yaml;
    use sprite_model::config::SpriteRef;

    use character_model::config::{
        CharacterControlTransitions, CharacterFrame, CharacterSequence, CharacterSequenceName,
    };

    const SEQUENCE_WITH_FRAMES_EMPTY: &str = "frames: []";
    const SEQUENCE_WITH_CONTROL_TRANSITIONS: &str = r#"---
transitions:
  press_defend: "stand_attack_1"

frames:
  - wait: 2
    sprite: { sheet: 0, index: 4 }
    transitions:
      press_attack: "stand_attack_0"
      hold_jump: { next: "jump" }
"#;

    #[test]
    fn sequence_with_empty_frames_list_deserializes_successfully() {
        let sequence = serde_yaml::from_str::<CharacterSequence>(SEQUENCE_WITH_FRAMES_EMPTY)
            .expect("Failed to deserialize sequence.");

        let expected = CharacterSequence::new(ObjectSequence::default(), None);
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_control_transitions() {
        let sequence = serde_yaml::from_str::<CharacterSequence>(SEQUENCE_WITH_CONTROL_TRANSITIONS)
            .expect("Failed to deserialize sequence.");

        let frames = vec![CharacterFrame::new(
            ObjectFrame {
                wait: Wait::new(2),
                sprite: SpriteRef::new(0, 4),
                ..Default::default()
            },
            CharacterControlTransitions {
                press_attack: Some(ControlTransition::SequenceNameString(
                    SequenceNameString::Name(CharacterSequenceName::StandAttack0),
                )),
                hold_jump: Some(ControlTransition::Single(ControlTransitionSingle {
                    next: SequenceNameString::Name(CharacterSequenceName::Jump),
                    requirements: vec![],
                })),
                ..Default::default()
            }, // kcov-ignore
        )];
        let character_control_transitions = CharacterControlTransitions {
            press_defend: Some(ControlTransition::SequenceNameString(
                SequenceNameString::Name(CharacterSequenceName::StandAttack1),
            )),
            ..Default::default()
        };
        let expected = CharacterSequence::new(
            ObjectSequence {
                next: SequenceEndTransition::None,
                frames,
                ..Default::default()
            },
            Some(character_control_transitions),
        );

        assert_eq!(expected, sequence);
    }
}
