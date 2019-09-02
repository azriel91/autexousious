use derive_new::new;
use object_model::config::{GameObjectSequence, ObjectSequence};
use serde::{Deserialize, Serialize};

use crate::config::{EnergyFrame, EnergySequenceName};

/// Represents an independent action sequence of an `Energy`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct EnergySequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub object_sequence: ObjectSequence<EnergySequenceName, EnergyFrame>,
}

impl GameObjectSequence for EnergySequence {
    type SequenceName = EnergySequenceName;
    type GameObjectFrame = EnergyFrame;

    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceName, Self::GameObjectFrame> {
        &self.object_sequence
    }
}

#[cfg(test)]
mod tests {
    use object_model::config::{ObjectFrame, ObjectSequence};
    use sequence_model::config::Wait;
    use serde_yaml;
    use sprite_model::config::SpriteRef;

    use super::EnergySequence;
    use crate::config::EnergyFrame;

    const SEQUENCE_WITH_FRAMES_EMPTY: &str = "frames: []";
    const SEQUENCE_WITH_FRAME: &str = r#"---
frames:
  - wait: 2
    sprite: { sheet: 0, index: 4 }
"#;

    #[test]
    fn sequence_with_empty_frames_list_deserializes_successfully() {
        let sequence = serde_yaml::from_str::<EnergySequence>(SEQUENCE_WITH_FRAMES_EMPTY)
            .expect("Failed to deserialize sequence.");

        let expected = EnergySequence::default();
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_frame() {
        let sequence = serde_yaml::from_str::<EnergySequence>(SEQUENCE_WITH_FRAME)
            .expect("Failed to deserialize sequence.");

        let frames = vec![EnergyFrame::new(ObjectFrame {
            wait: Wait::new(2),
            sprite: SpriteRef::new(0, 4),
            ..Default::default()
        })];
        let expected = EnergySequence::new(ObjectSequence {
            frames,
            ..Default::default()
        });

        assert_eq!(expected, sequence);
    }
}
