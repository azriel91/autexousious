//! User defined configuration types for spawns.

pub use self::{spawn::Spawn, spawns::Spawns};

mod spawn;
mod spawns;

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use asset_model::config::AssetSlug;
    use kinematic_model::config::{Position, Velocity};
    use serde::{Deserialize, Serialize};
    use serde_yaml;

    use super::{Spawn, Spawns};

    const SPAWNS_YAML: &str = r#"
spawns:
  - { object: "default/fireball" }
  - object: "default/fireball"
    position: { x: -1, y: 2, z: 3 }
    velocity: { x: -4, y: 5 }
    sequence: "sequence_name_string"
"#;

    #[derive(Debug, Deserialize, Serialize)]
    struct Config {
        spawns: Spawns,
    }

    #[test]
    fn deserialize_spawns() {
        let config =
            serde_yaml::from_str::<Config>(SPAWNS_YAML).expect("Failed to deserialize `Spawns`.");
        let spawns = config.spawns;

        let asset_slug = AssetSlug::from_str("default/fireball")
            .expect("Expected `default/fireball` to be a valid asset slug.");
        assert_eq!(
            Spawns::new(vec![
                Spawn::new(
                    asset_slug.clone(),
                    Position::<i32>::from((0, 0, 0)),
                    Velocity::<i32>::from((0, 0, 0)),
                    None,
                ),
                Spawn::new(
                    asset_slug,
                    Position::<i32>::from((-1, 2, 3)),
                    Velocity::<i32>::from((-4, 5, 0)),
                    Some(String::from("sequence_name_string")),
                )
            ]),
            spawns
        );
    }
}
