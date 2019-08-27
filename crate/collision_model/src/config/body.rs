use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};
use shape_model::Volume;

/// Hittable volumes of an interactable object.
#[derive(
    Asset, Clone, Debug, Default, Deref, DerefMut, Deserialize, Hash, PartialEq, Eq, Serialize, new,
)]
pub struct Body(
    /// Backing vector of `Volume`s.
    #[serde(default)]
    pub Vec<Volume>,
);

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_yaml;
    use shape_model::{Axis, Volume};

    use super::Body;

    const BODY_ALL_SPECIFIED: &str = r#"---
body:
  - { box: { x: -1, y: -2, z: -3, w: 11, h: 12, d: 13 } }
  - { cylinder: { axis: "x", center: -4, r: 14, l: 24 } }
  - { cylinder: { axis: "y", center: -5, r: 15, l: 25 } }
  - { cylinder: { axis: "z", center: -6, r: 16, l: 26 } }
  - { sphere: { x: -7, y: -8, z: -9, r: 17 } }
"#;
    const BODY_MINIMUM_SPECIFIED: &str = r#"---
body:
  - { box: { x: -1, y: -2, w: 11, h: 12 } }
  - { sphere: { x: -7, y: -8, r: 17 } }
"#;

    #[test]
    fn body_specify_all_fields() {
        let frame = serde_yaml::from_str::<BodyFrame>(BODY_ALL_SPECIFIED)
            .expect("Failed to deserialize frame.");

        let body_volumes = vec![
            Volume::Box {
                x: -1,
                y: -2,
                z: -3,
                w: 11,
                h: 12,
                d: 13,
            },
            Volume::Cylinder {
                axis: Axis::X,
                center: -4,
                r: 14,
                l: 24,
            },
            Volume::Cylinder {
                axis: Axis::Y,
                center: -5,
                r: 15,
                l: 25,
            },
            Volume::Cylinder {
                axis: Axis::Z,
                center: -6,
                r: 16,
                l: 26,
            },
            Volume::Sphere {
                x: -7,
                y: -8,
                z: -9,
                r: 17,
            },
        ];
        assert_eq!(Body::new(body_volumes), frame.body);
    }

    #[test]
    fn body_specify_minimum_fields() {
        let frame = serde_yaml::from_str::<BodyFrame>(BODY_MINIMUM_SPECIFIED)
            .expect("Failed to deserialize frame.");

        let body_volumes = vec![
            Volume::Box {
                x: -1,
                y: -2,
                z: 0,
                w: 11,
                h: 12,
                d: 26,
            },
            Volume::Sphere {
                x: -7,
                y: -8,
                z: 0,
                r: 17,
            },
        ];
        assert_eq!(Body::new(body_volumes), frame.body);
    }

    /// Needed because the YAML deserializer does not support deserializing values directly.
    #[derive(Debug, Deserialize)]
    struct BodyFrame {
        body: Body,
    }
}
