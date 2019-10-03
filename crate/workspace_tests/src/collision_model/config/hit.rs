#[cfg(test)]
mod test {
    use kinematic_model::config::Acceleration;
    use object_status_model::config::StunPoints;
    use serde_yaml;

    use collision_model::config::{Hit, HitLimit, HitRepeatDelay};

    const HIT_YAML: &str = r#"---
repeat_delay: 1
hit_limit: 2
hp_damage: 3
sp_damage: 4
stun: 5
acceleration: { x: -1, y: 2 }
"#;

    #[test]
    fn deserialize_hit() {
        let hit_deserialized =
            serde_yaml::from_str::<Hit>(HIT_YAML).expect("Failed to deserialize `Hit`.");

        let expected = Hit::new(
            HitRepeatDelay::new(1),
            HitLimit::Limit(2),
            3,
            4,
            StunPoints::new(5),
            Acceleration::new(-1, 2, 0),
        );

        assert_eq!(expected, hit_deserialized);
    }
}
