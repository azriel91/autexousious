use amethyst::{
    assets::{Asset, Handle, ProcessingState, Result as AssetsResult},
    ecs::VecStorage,
};
use derivative::Derivative;
use derive_new::new;

use crate::config::Interaction;

/// Frame for an interactable object.
#[derive(Clone, Debug, Derivative, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[derivative(Default)]
pub struct InteractionFrame {
    /// Effects on other objects.
    #[serde(default)]
    pub interactions: Option<Vec<Interaction>>,
    /// Number of ticks to wait before the sequence switches to the next frame.
    #[serde(default)]
    pub wait: u32,
}

impl Asset for InteractionFrame {
    const NAME: &'static str = "collision_model::config::InteractionFrame";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<InteractionFrame> for AssetsResult<ProcessingState<InteractionFrame>> {
    fn from(
        interaction_frame: InteractionFrame,
    ) -> AssetsResult<ProcessingState<InteractionFrame>> {
        Ok(ProcessingState::Loaded(interaction_frame))
    }
}

#[cfg(test)]
mod tests {
    use shape_model::Volume;
    use toml;

    use super::InteractionFrame;
    use crate::config::Interaction;

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
    fn itr_physical_specify_all_fields() {
        let frame = toml::from_str::<InteractionFrame>(ITR_PHYSICAL_ALL_SPECIFIED)
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
        assert_eq!(InteractionFrame::new(Some(interactions), 0), frame);
    }

    #[test]
    fn itr_physical_specify_minimum_fields() {
        let frame = toml::from_str::<InteractionFrame>(ITR_PHYSICAL_MINIMUM_SPECIFIED)
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
        assert_eq!(InteractionFrame::new(Some(interactions), 0), frame);
    }
}
