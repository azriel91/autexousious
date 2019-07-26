use derive_new::new;
use object_model::config::{GameObjectSequence, ObjectSequence};
use serde::{Deserialize, Serialize};

use crate::config::{CharacterFrame, CharacterSequenceId};

/// Represents an independent action sequence of a character.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct CharacterSequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub object_sequence: ObjectSequence<CharacterSequenceId, CharacterFrame>,
}

impl GameObjectSequence for CharacterSequence {
    type SequenceId = CharacterSequenceId;
    type GameObjectFrame = CharacterFrame;

    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceId, Self::GameObjectFrame> {
        &self.object_sequence
    }
}

#[cfg(test)]
mod tests {
    use object_model::config::{ObjectFrame, ObjectSequence};
    use sequence_model::config::{
        ControlTransition, ControlTransitionSingle, SequenceEndTransition, Wait,
    };
    use sprite_model::config::SpriteRef;
    use toml;

    use super::CharacterSequence;
    use crate::config::{CharacterControlTransitions, CharacterFrame, CharacterSequenceId};

    const SEQUENCE_WITH_FRAMES_EMPTY: &str = "frames = []";
    const SEQUENCE_WITH_CONTROL_TRANSITIONS: &str = r#"
        [[frames]]
        wait = 2
        sprite = { sheet = 0, index = 4 }

        [frames.transitions]
          press_attack = "stand_attack_0"
          hold_jump = { next = "jump" }
    "#;

    #[test]
    fn sequence_with_empty_frames_list_deserializes_successfully() {
        let sequence = toml::from_str::<CharacterSequence>(SEQUENCE_WITH_FRAMES_EMPTY)
            .expect("Failed to deserialize sequence.");

        let expected =
            CharacterSequence::new(ObjectSequence::new(SequenceEndTransition::None, vec![]));
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_control_transitions() {
        let sequence = toml::from_str::<CharacterSequence>(SEQUENCE_WITH_CONTROL_TRANSITIONS)
            .expect("Failed to deserialize sequence.");

        let frames = vec![CharacterFrame::new(
            ObjectFrame {
                wait: Wait::new(2),
                sprite: SpriteRef::new(0, 4),
                ..Default::default()
            },
            CharacterControlTransitions {
                press_attack: Some(ControlTransition::SequenceId(
                    CharacterSequenceId::StandAttack0,
                )),
                hold_jump: Some(ControlTransition::Single(ControlTransitionSingle {
                    next: CharacterSequenceId::Jump,
                    requirements: vec![],
                })),
                ..Default::default()
            }, // kcov-ignore
        )];
        let expected =
            CharacterSequence::new(ObjectSequence::new(SequenceEndTransition::None, frames));

        assert_eq!(expected, sequence);
    }
}
