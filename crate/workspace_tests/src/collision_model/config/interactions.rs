#[cfg(test)]
mod tests {
    use kinematic_model::config::Acceleration;
    use object_status_model::config::StunPoints;
    use serde::Deserialize;
    use serde_yaml;
    use shape_model::Volume;

    use collision_model::config::{
        Hit, HitLimit, HitRepeatDelay, Interaction, InteractionKind, Interactions,
    };

    const ITR_PHYSICAL_ALL_SPECIFIED: &str = r#"---
interactions:
  - hit:
      repeat_delay: 5
      hit_limit: "unlimited"
      hp_damage: 40
      sp_damage: 50
      stun: 33
      acceleration: { x: -1, y: 2 }
    bounds: [{ sphere: { x: 1, y: 1, r: 1 } }]
    multiple: true
"#;
    const ITR_PHYSICAL_MINIMUM_SPECIFIED: &str = r#"---
interactions:
  - { hit: {}, bounds: [{ sphere: { x: 1, y: 1, r: 1 } }] }
"#;
    const ITR_PHYSICAL_HIT_LIMIT: &str = r#"---
interactions:
  - { hit: { hit_limit: 2 }, bounds: [{ sphere: { x: 1, y: 1, r: 1 } }] }
"#;

    #[test]
    fn itr_physical_specify_all_fields() {
        let frame = serde_yaml::from_str::<InteractionsFrame>(ITR_PHYSICAL_ALL_SPECIFIED)
            .expect("Failed to deserialize frame.");

        let interactions = vec![Interaction {
            kind: InteractionKind::Hit(Hit {
                repeat_delay: HitRepeatDelay::new(5),
                hit_limit: HitLimit::Unlimited,
                hp_damage: 40,
                sp_damage: 50,
                stun: StunPoints::new(33),
                acceleration: Acceleration::new(-1, 2, 0),
            }),
            bounds: vec![Volume::Sphere {
                x: 1,
                y: 1,
                z: 0,
                r: 1,
            }],
            multiple: true,
        }];
        assert_eq!(Interactions::new(interactions), frame.interactions);
    }

    #[test]
    fn itr_physical_specify_minimum_fields() {
        let frame = serde_yaml::from_str::<InteractionsFrame>(ITR_PHYSICAL_MINIMUM_SPECIFIED)
            .expect("Failed to deserialize frame.");

        let interactions = vec![Interaction {
            bounds: vec![Volume::Sphere {
                x: 1,
                y: 1,
                z: 0,
                r: 1,
            }],
            kind: Default::default(),
            multiple: Default::default(),
        }]; // kcov-ignore
        assert_eq!(Interactions::new(interactions), frame.interactions);
    }

    #[test]
    fn itr_physical_specify_hit_limit() {
        let frame = serde_yaml::from_str::<InteractionsFrame>(ITR_PHYSICAL_HIT_LIMIT)
            .expect("Failed to deserialize frame.");

        let interactions = vec![Interaction {
            bounds: vec![Volume::Sphere {
                x: 1,
                y: 1,
                z: 0,
                r: 1,
            }],
            kind: InteractionKind::Hit(Hit {
                hit_limit: HitLimit::Limit(2),
                ..Default::default()
            }),
            multiple: Default::default(),
        }]; // kcov-ignore
        assert_eq!(Interactions::new(interactions), frame.interactions);
    }

    /// Needed because the YAML deserializer does not support deserializing
    /// values directly.
    #[derive(Debug, Deserialize)]
    struct InteractionsFrame {
        interactions: Interactions,
    }
}
