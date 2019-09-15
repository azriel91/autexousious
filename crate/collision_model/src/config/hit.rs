use derive_new::new;
use kinematic_model::config::Acceleration;
use object_status_model::config::StunPoints;
use serde::{Deserialize, Serialize};

use crate::config::{HitLimit, HitRepeatDelay};

/// Configuration of a hit interaction.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct Hit {
    /// Number of ticks to wait before another hit may occur.
    pub repeat_delay: HitRepeatDelay,
    /// Number of objects a `Hit` may collide with.
    pub hit_limit: HitLimit,
    /// Amount of health points (HP) to subtract on collision.
    pub hp_damage: u32,
    /// Amount of skill points (SP) to subtract on collision.
    pub sp_damage: u32,
    /// Amount of stun points to inflict on collision.
    pub stun: StunPoints,
    /// Acceleration to inflict on collision.
    pub acceleration: Acceleration<i32>,
}

#[cfg(test)]
mod test {
    use kinematic_model::config::Acceleration;
    use object_status_model::config::StunPoints;
    use serde_yaml;

    use super::Hit;
    use crate::config::{HitLimit, HitRepeatDelay};

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
