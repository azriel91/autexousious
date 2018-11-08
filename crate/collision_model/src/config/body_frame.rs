use amethyst::{
    assets::{Asset, Handle, ProcessingState, Result as AssetsResult},
    ecs::VecStorage,
};
use shape_model::Volume;

/// Frame for an interactable object.
#[derive(Clone, Debug, Derivative, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[derivative(Default)]
pub struct BodyFrame {
    /// Hittable volumes of the object.
    #[serde(default)]
    pub body: Option<Vec<Volume>>,
    /// Number of ticks to wait before the sequence switches to the next frame.
    #[serde(default)]
    pub wait: u32,
}

impl Asset for BodyFrame {
    const NAME: &'static str = "collision_model::config::BodyFrame";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<BodyFrame> for AssetsResult<ProcessingState<BodyFrame>> {
    fn from(body_frame: BodyFrame) -> AssetsResult<ProcessingState<BodyFrame>> {
        Ok(ProcessingState::Loaded(body_frame))
    }
}

#[cfg(test)]
mod tests {
    use shape_model::{Axis, Volume};
    use toml;

    use super::BodyFrame;

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

    #[test]
    fn body_specify_all_fields() {
        let frame =
            toml::from_str::<BodyFrame>(BODY_ALL_SPECIFIED).expect("Failed to deserialize frame.");

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
        assert_eq!(BodyFrame::new(Some(body_volumes), 0), frame);
    }

    #[test]
    fn body_specify_minimum_fields() {
        let frame = toml::from_str::<BodyFrame>(BODY_MINIMUM_SPECIFIED)
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
        assert_eq!(BodyFrame::new(Some(body_volumes), 0), frame);
    }
}
