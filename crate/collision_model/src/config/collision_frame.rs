use amethyst::{
    assets::{Asset, Handle},
    ecs::VecStorage,
};
use shape_model::Volume;

use config::Interaction;

/// Frame for an interactable object.
#[derive(Clone, Debug, Derivative, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[derivative(Default)]
pub struct CollisionFrame {
    /// Hittable volumes of the object.
    #[serde(default)]
    pub body: Option<Vec<Volume>>,
    /// Effects on other objects.
    #[serde(default)]
    pub interactions: Option<Vec<Interaction>>,
    /// Number of ticks to wait before the sequence switches to the next frame.
    #[serde(default)]
    pub wait: u32,
}

impl Asset for CollisionFrame {
    const NAME: &'static str = "collision_model::config::CollisionFrame";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

#[cfg(test)]
mod tests {
    use shape_model::{Axis, Volume};
    use toml;

    use super::CollisionFrame;
    use config::Interaction;

    const BODY_ALL_SPECIFIED: &str = r#"
        body = [
          { box = { x = -1, y = -2, z = -3, w = 11, h = 12, d = 13 } },
          { cylinder = { axis = "x", center = -4, r = 14, l = 24 } },
          { cylinder = { axis = "y", center = -5, r = 15, l = 25 } },
          { cylinder = { axis = "z", center = -6, r = 16, l = 26 } },
          { sphere = { x = -7, y = -8, z = -9, r = 17 } },
        ]
    "#;
    const BODY_MINIMUM_SPECIFIED: &str = r#"
        body = [
          { box = { x = -1, y = -2, w = 11, h = 12 } },
          { sphere = { x = -7, y = -8, r = 17 } },
        ]
    "#;
    const ITR_PHYSICAL_ALL_SPECIFIED: &str = r#"
        interactions = [
          { physical = { bounds = [{ sphere = { x = 1, y = 1, r = 1 } }], hp_damage = 40, sp_damage = 50, multiple = true } },
        ]
    "#;
    const ITR_PHYSICAL_MINIMUM_SPECIFIED: &str = r#"
        interactions = [
          { physical = { bounds = [{ sphere = { x = 1, y = 1, r = 1 } }] } },
        ]
    "#;

    #[test]
    fn body_specify_all_fields() {
        let frame = toml::from_str::<CollisionFrame>(BODY_ALL_SPECIFIED)
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
        assert_eq!(CollisionFrame::new(Some(body_volumes), None, 0), frame);
    }

    #[test]
    fn body_specify_minimum_fields() {
        let frame = toml::from_str::<CollisionFrame>(BODY_MINIMUM_SPECIFIED)
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
        assert_eq!(CollisionFrame::new(Some(body_volumes), None, 0), frame);
    }

    #[test]
    fn itr_physical_specify_all_fields() {
        let frame = toml::from_str::<CollisionFrame>(ITR_PHYSICAL_ALL_SPECIFIED)
            .expect("Failed to deserialize frame.");

        let interactions = vec![Interaction::Physical {
            bounds: vec![Volume::Sphere {
                x: 1,
                y: 1,
                z: 0,
                r: 1,
            }],
            hp_damage: 40,
            sp_damage: 50,
            multiple: true,
        }];
        assert_eq!(CollisionFrame::new(None, Some(interactions), 0), frame);
    }

    #[test]
    fn itr_physical_specify_minimum_fields() {
        let frame = toml::from_str::<CollisionFrame>(ITR_PHYSICAL_MINIMUM_SPECIFIED)
            .expect("Failed to deserialize frame.");

        let interactions = vec![Interaction::Physical {
            bounds: vec![Volume::Sphere {
                x: 1,
                y: 1,
                z: 0,
                r: 1,
            }],
            hp_damage: 0,
            sp_damage: 0,
            multiple: false,
        }];
        assert_eq!(CollisionFrame::new(None, Some(interactions), 0), frame);
    }
}
