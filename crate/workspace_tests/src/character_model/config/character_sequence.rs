#[cfg(test)]
mod tests {
    use input_reaction_model::config::{
        InputReaction, InputReactionAppEvents, InputReactionSingle,
    };
    use object_model::config::{ObjectFrame, ObjectSequence};
    use sequence_model::config::{Sequence, SequenceEndTransition, SequenceNameString, Wait};
    use serde_yaml;
    use sprite_model::config::SpriteRef;

    use character_model::config::{
        CharacterFrame, CharacterInputReactions, CharacterIrr, CharacterSequence,
        CharacterSequenceName,
    };

    const SEQUENCE_WITH_FRAMES_EMPTY: &str = "frames: []";
    const SEQUENCE_WITH_INPUT_REACTIONS: &str = r#"---
input_reactions:
  press_defend: "stand_attack_1"

frames:
  - wait: 2
    sprite: { sheet: 0, index: 4 }
    input_reactions:
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
    fn sequence_with_input_reactions() {
        let sequence = serde_yaml::from_str::<CharacterSequence>(SEQUENCE_WITH_INPUT_REACTIONS)
            .expect("Failed to deserialize sequence.");

        let frames = vec![CharacterFrame::new(
            ObjectFrame {
                wait: Wait::new(2),
                sprite: SpriteRef::new(0, 4),
                ..Default::default()
            },
            CharacterInputReactions {
                press_attack: Some(InputReaction::SequenceNameString(SequenceNameString::Name(
                    CharacterSequenceName::StandAttack0,
                ))),
                hold_jump: Some(InputReaction::Single(InputReactionSingle {
                    next: SequenceNameString::Name(CharacterSequenceName::Jump),
                    events: InputReactionAppEvents::default(),
                    requirement: CharacterIrr::default(),
                })),
                ..Default::default()
            }, // kcov-ignore
        )];
        let character_input_reactions = CharacterInputReactions {
            press_defend: Some(InputReaction::SequenceNameString(SequenceNameString::Name(
                CharacterSequenceName::StandAttack1,
            ))),
            ..Default::default()
        };
        let expected = CharacterSequence::new(
            ObjectSequence {
                sequence: Sequence {
                    next: SequenceEndTransition::None,
                    frames,
                },
                ..Default::default()
            },
            Some(character_input_reactions),
        );

        assert_eq!(expected, sequence);
    }
}
