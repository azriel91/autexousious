#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use amethyst::ecs::{storage::DenseVecStorage, Component};
    use asset_model::config::AssetSlug;
    use collision_model::config::{
        Body, Hit, HitRepeatDelay, Interaction, InteractionKind, Interactions,
    };
    use derivative::Derivative;
    use kinematic_model::config::{
        ObjectAcceleration, ObjectAccelerationKind, ObjectAccelerationValue,
        ObjectAccelerationValueExpr, ObjectAccelerationValueMultiplier, Position, Velocity,
    };
    use sequence_model::config::{SequenceEndTransition, SequenceName, SequenceNameString, Wait};
    use serde::{Deserialize, Serialize};
    use serde_yaml;
    use shape_model::Volume;
    use spawn_model::config::{Spawn, Spawns};
    use sprite_model::config::SpriteRef;
    use strum_macros::{Display, EnumString, IntoStaticStr};

    use object_model::config::{ObjectFrame, ObjectSequence};

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
    const SEQUENCE_WITH_SOUND: &str = r#"---
frames:
  - sound: "path/to/sound.wav"
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
    const SEQUENCE_WITH_ACCELERATION_DEFAULTS: &str = r#"---
frames:
  - acceleration: { kind: "continuous" }
"#;

    const SEQUENCE_WITH_ACCELERATION_FULL: &str = r#"---
acceleration:
  kind: "continuous"
  x: { value: -5.0 }
  y: -2.5
  z: { multiplier: "z_axis", value: -3.0 }
frames:
  - acceleration:
      kind: "once"
      x: { value: 5.0 }
      y: 2.5
      z: { multiplier: "z_axis", value: 3.0 }
"#;

    #[test]
    fn sequence_with_empty_frames_list_deserializes_successfully() {
        let sequence =
            serde_yaml::from_str::<ObjectSequence<TestSeqName>>(SEQUENCE_WITH_FRAMES_EMPTY)
                .expect("Failed to deserialize sequence.");

        let expected = ObjectSequence::default();
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_frames() {
        let sequence = serde_yaml::from_str::<ObjectSequence<TestSeqName>>(SEQUENCE_WITH_FRAMES)
            .expect("Failed to deserialize sequence.");

        let frames = vec![
            ObjectFrame {
                wait: Wait::new(2),
                sprite: SpriteRef::new(0, 4),
                ..Default::default()
            },
            ObjectFrame {
                wait: Wait::new(2),
                sprite: SpriteRef::new(0, 5),
                ..Default::default()
            },
            ObjectFrame {
                wait: Wait::new(1),
                sprite: SpriteRef::new(1, 6),
                ..Default::default()
            },
            ObjectFrame {
                wait: Wait::new(1),
                sprite: SpriteRef::new(1, 7),
                ..Default::default()
            },
            ObjectFrame {
                wait: Wait::new(2),
                sprite: SpriteRef::new(0, 6),
                ..Default::default()
            },
            ObjectFrame {
                wait: Wait::new(2),
                sprite: SpriteRef::new(0, 5),
                ..Default::default()
            },
        ];
        let expected = ObjectSequence {
            next: SequenceEndTransition::SequenceName(SequenceNameString::Name(TestSeqName::Boo)),
            frames,
            ..Default::default()
        };
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_sound() {
        let sequence = serde_yaml::from_str::<ObjectSequence<TestSeqName>>(SEQUENCE_WITH_SOUND)
            .expect("Failed to deserialize sequence.");

        let frames = vec![ObjectFrame {
            wait: Wait::new(0),
            sound: Some(PathBuf::from("path/to/sound.wav")),
            ..Default::default()
        }];
        let expected = ObjectSequence {
            next: SequenceEndTransition::None,
            frames,
            ..Default::default()
        };
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
        let frames = vec![ObjectFrame {
            wait: Wait::new(0),
            body: Body::new(body_volumes),
            ..Default::default()
        }];
        let expected = ObjectSequence {
            next: SequenceEndTransition::None,
            frames,
            ..Default::default()
        };
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_itr() {
        let sequence = serde_yaml::from_str::<ObjectSequence<TestSeqName>>(SEQUENCE_WITH_ITR)
            .expect("Failed to deserialize sequence.");

        let interactions = vec![Interaction {
            kind: InteractionKind::Hit(Hit {
                repeat_delay: HitRepeatDelay::new(5),
                ..Default::default()
            }),
            bounds: vec![Volume::Sphere {
                x: 1,
                y: 1,
                z: 0,
                r: 1,
            }],
            multiple: false,
        }];
        let frames = vec![ObjectFrame {
            wait: Wait::new(0),
            interactions: Interactions::new(interactions),
            ..Default::default()
        }];
        let expected = ObjectSequence {
            next: SequenceEndTransition::None,
            frames,
            ..Default::default()
        };
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
            None,
        )];
        let frames = vec![ObjectFrame {
            wait: Wait::new(0),
            spawns: Spawns::new(spawns),
            ..Default::default()
        }];
        let expected = ObjectSequence {
            next: SequenceEndTransition::None,
            frames,
            ..Default::default()
        };
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_acceleration_defaults() {
        let sequence = serde_yaml::from_str::<ObjectSequence<TestSeqName>>(
            SEQUENCE_WITH_ACCELERATION_DEFAULTS,
        )
        .expect("Failed to deserialize sequence.");

        let frames = vec![ObjectFrame {
            wait: Wait::new(0),
            acceleration: Some(ObjectAcceleration {
                kind: ObjectAccelerationKind::Continuous,
                ..Default::default()
            }),
            ..Default::default()
        }];
        let expected = ObjectSequence {
            next: SequenceEndTransition::None,
            frames,
            ..Default::default()
        };
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_acceleration_full() {
        let sequence =
            serde_yaml::from_str::<ObjectSequence<TestSeqName>>(SEQUENCE_WITH_ACCELERATION_FULL)
                .expect("Failed to deserialize sequence.");

        let frames = vec![ObjectFrame {
            wait: Wait::new(0),
            acceleration: Some(ObjectAcceleration {
                kind: ObjectAccelerationKind::Once,
                x: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                    multiplier: ObjectAccelerationValueMultiplier::One,
                    value: 5.,
                }),
                y: ObjectAccelerationValue::Const(2.5),
                z: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                    multiplier: ObjectAccelerationValueMultiplier::ZAxis,
                    value: 3.,
                }),
            }),
            ..Default::default()
        }];
        let expected = ObjectSequence {
            next: SequenceEndTransition::None,
            acceleration: Some(ObjectAcceleration {
                kind: ObjectAccelerationKind::Continuous,
                x: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                    multiplier: ObjectAccelerationValueMultiplier::One,
                    value: -5.,
                }),
                y: ObjectAccelerationValue::Const(-2.5),
                z: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                    multiplier: ObjectAccelerationValueMultiplier::ZAxis,
                    value: -3.,
                }),
            }),
            frames,
        };
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
