//! Configuration types for object sequences.
//!
//! A sequence is an independent grouping of frames, which contains not only animation information,
//! but also the collision zones, interaction, and effects.
//!
//! Sequences are shared by different object types, and are genericized by the sequence name. This is
//! because different object types have different valid sequence names, and we want to be able to
//! define this at compile time rather than needing to process this at run time.

use derive_new::new;
use sequence_model::config::{SequenceEndTransition, SequenceName};
use serde::{Deserialize, Serialize};

use crate::config::{GameObjectFrame, ObjectFrame};

/// Represents an independent action sequence of an object.
///
/// This carries the information necessary for an `Animation`, as well as the effects and
/// interactions that happen during each frame of that animation.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ObjectSequence<SeqName, Frame = ObjectFrame>
where
    SeqName: SequenceName,
    Frame: GameObjectFrame,
{
    /// Name of the sequence to switch to after this one has completed.
    ///
    /// Note: This may not be immediately after the last frame of the sequence. For example, a
    /// character that is in mid-air should remain in the last frame until it lands on the ground.
    #[serde(default)]
    pub next: SequenceEndTransition<SeqName>,
    /// Key frames in the animation sequence.
    pub frames: Vec<Frame>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use amethyst::ecs::{storage::DenseVecStorage, Component};
    use asset_model::config::AssetSlug;
    use collision_model::config::{
        Body, Hit, HitLimit, HitRepeatDelay, Interaction, InteractionKind, Interactions,
    };
    use derivative::Derivative;
    use kinematic_model::config::{Position, Velocity};
    use object_status_model::config::StunPoints;
    use sequence_model::config::{SequenceEndTransition, SequenceName, Wait};
    use serde::{Deserialize, Serialize};
    use serde_yaml;
    use shape_model::Volume;
    use spawn_model::config::{Spawn, Spawns};
    use specs_derive::Component;
    use sprite_model::config::SpriteRef;
    use strum_macros::{Display, EnumString, IntoStaticStr};

    use super::ObjectSequence;
    use crate::config::ObjectFrame;

    const SEQUENCE_WITH_FRAMES: &str = r#"---
next: "boo"
frames:
  - { wait: 2, sprite: { sheet: 0, index: 4 } }
  - { wait: 2, sprite: { sheet: 0, index: 5 } }
  - { wait: 1, sprite: { sheet: 1, index: 6 } }
  - { wait: 1, sprite: { sheet: 1, index: 7 } }
  - { wait: 2, sprite: { sheet: 0, index: 6 } }
  - { wait: 2, sprite: { sheet: 0, index: 5 } }
"#;
    const SEQUENCE_WITH_FRAMES_EMPTY: &str = r#"---
frames: []
"#;
    const SEQUENCE_WITH_BODY: &str = r#"---
frames:
  - body:
      - { box: { x: -1, y: -2, z: -3, w: 11, h: 12, d: 13 } }
      - { sphere: { x: -7, y: -8, z: -9, r: 17 } }
"#;
    const SEQUENCE_WITH_ITR: &str = r#"---
frames:
  - interactions:
      - { hit: { repeat_delay: 5 }, bounds: [{ sphere: { x: 1, y: 1, r: 1 } }] }
"#;
    const SEQUENCE_WITH_SPAWNS: &str = r#"---
frames:
  - spawns: [{ object: "default/fireball" }]
"#;

    #[test]
    fn sequence_with_empty_frames_list_deserializes_successfully() {
        let sequence =
            serde_yaml::from_str::<ObjectSequence<TestSeqName>>(SEQUENCE_WITH_FRAMES_EMPTY)
                .expect("Failed to deserialize sequence.");

        let expected = ObjectSequence::new(SequenceEndTransition::None, vec![]);
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_frames() {
        let sequence = serde_yaml::from_str::<ObjectSequence<TestSeqName>>(SEQUENCE_WITH_FRAMES)
            .expect("Failed to deserialize sequence.");

        let frames = vec![
            ObjectFrame::new(
                Wait::new(2),
                SpriteRef::new(0, 4),
                Body::default(),
                Interactions::default(),
                Spawns::default(),
            ),
            ObjectFrame::new(
                Wait::new(2),
                SpriteRef::new(0, 5),
                Body::default(),
                Interactions::default(),
                Spawns::default(),
            ),
            ObjectFrame::new(
                Wait::new(1),
                SpriteRef::new(1, 6),
                Body::default(),
                Interactions::default(),
                Spawns::default(),
            ),
            ObjectFrame::new(
                Wait::new(1),
                SpriteRef::new(1, 7),
                Body::default(),
                Interactions::default(),
                Spawns::default(),
            ),
            ObjectFrame::new(
                Wait::new(2),
                SpriteRef::new(0, 6),
                Body::default(),
                Interactions::default(),
                Spawns::default(),
            ),
            ObjectFrame::new(
                Wait::new(2),
                SpriteRef::new(0, 5),
                Body::default(),
                Interactions::default(),
                Spawns::default(),
            ),
        ];
        let expected = ObjectSequence::new(
            SequenceEndTransition::SequenceName(TestSeqName::Boo),
            frames,
        );
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_body() {
        let sequence = serde_yaml::from_str::<ObjectSequence<TestSeqName>>(SEQUENCE_WITH_BODY)
            .expect("Failed to deserialize sequence.");

        let body_volumes = vec![
            Volume::Box {
                x: -1,
                y: -2,
                z: -3,
                w: 11,
                h: 12,
                d: 13,
            },
            Volume::Sphere {
                x: -7,
                y: -8,
                z: -9,
                r: 17,
            },
        ];
        let frames = vec![ObjectFrame::new(
            Wait::new(0),
            SpriteRef::default(),
            Body::new(body_volumes),
            Interactions::default(),
            Spawns::default(),
        )];
        let expected = ObjectSequence::new(SequenceEndTransition::None, frames);
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_itr() {
        let sequence = serde_yaml::from_str::<ObjectSequence<TestSeqName>>(SEQUENCE_WITH_ITR)
            .expect("Failed to deserialize sequence.");

        let interactions = vec![Interaction {
            kind: InteractionKind::Hit(Hit {
                repeat_delay: HitRepeatDelay::new(5),
                hit_limit: HitLimit::default(),
                hp_damage: 0,
                sp_damage: 0,
                stun: StunPoints::default(),
            }),
            bounds: vec![Volume::Sphere {
                x: 1,
                y: 1,
                z: 0,
                r: 1,
            }],
            multiple: false,
        }];
        let frames = vec![ObjectFrame::new(
            Wait::new(0),
            SpriteRef::default(),
            Body::default(),
            Interactions::new(interactions),
            Spawns::default(),
        )];
        let expected = ObjectSequence::new(SequenceEndTransition::None, frames);
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_spawns() {
        let sequence = serde_yaml::from_str::<ObjectSequence<TestSeqName>>(SEQUENCE_WITH_SPAWNS)
            .expect("Failed to deserialize sequence.");

        let asset_slug = AssetSlug::from_str("default/fireball")
            .expect("Expected `default/fireball` to be a valid asset slug.");
        let spawns = vec![Spawn::new(
            asset_slug,
            Position::<i32>::from((0, 0, 0)),
            Velocity::<i32>::from((0, 0, 0)),
        )];
        let frames = vec![ObjectFrame::new(
            Wait::new(0),
            SpriteRef::default(),
            Body::default(),
            Interactions::default(),
            Spawns::new(spawns),
        )];
        let expected = ObjectSequence::new(SequenceEndTransition::None, frames);
        assert_eq!(expected, sequence);
    }

    #[derive(
        Clone,
        Component,
        Copy,
        Debug,
        Derivative,
        Deserialize,
        Display,
        EnumString,
        IntoStaticStr,
        PartialEq,
        Eq,
        Hash,
        Serialize,
    )]
    #[derivative(Default)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    enum TestSeqName {
        #[derivative(Default)]
        Boo,
    }
    impl SequenceName for TestSeqName {}
}
