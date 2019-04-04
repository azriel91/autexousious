use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::VecStorage,
    Error,
};
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::Interaction;

/// Effects on other objects.
#[derive(
    Clone, Debug, Default, Deref, DerefMut, Deserialize, Hash, PartialEq, Eq, Serialize, new,
)]
pub struct Interactions(
    /// Backing vector of `Interaction`s.
    #[serde(default)]
    pub Vec<Interaction>,
);

impl Asset for Interactions {
    const NAME: &'static str = concat!(module_path!(), "::", stringify!(Interactions));
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<Interactions> for Result<ProcessingState<Interactions>, Error> {
    fn from(interactions: Interactions) -> Result<ProcessingState<Interactions>, Error> {
        Ok(ProcessingState::Loaded(interactions))
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use shape_model::Volume;
    use toml;

    use super::Interactions;
    use crate::config::{Hit, HitRepeatDelay, Interaction, InteractionKind};

    const ITR_PHYSICAL_ALL_SPECIFIED: &str = "
        interactions = [
          { hit = { repeat_delay = 5, hp_damage = 40, sp_damage = 50 }, \
            bounds = [{ sphere = { x = 1, y = 1, r = 1 } }], multiple = true },
        ]
    ";
    const ITR_PHYSICAL_MINIMUM_SPECIFIED: &str = r#"
        interactions = [
          { hit = {}, bounds = [{ sphere = { x = 1, y = 1, r = 1 } }] },
        ]
    "#;

    #[test]
    fn itr_physical_specify_all_fields() {
        let frame = toml::from_str::<InteractionsFrame>(ITR_PHYSICAL_ALL_SPECIFIED)
            .expect("Failed to deserialize frame.");

        let interactions = vec![Interaction {
            kind: InteractionKind::Hit(Hit {
                hp_damage: 40,
                sp_damage: 50,
                repeat_delay: HitRepeatDelay::new(5),
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
        let frame = toml::from_str::<InteractionsFrame>(ITR_PHYSICAL_MINIMUM_SPECIFIED)
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
        }];
        assert_eq!(Interactions::new(interactions), frame.interactions);
    }

    /// Needed because the TOML deserializer does not support deserializing values directly.
    #[derive(Debug, Deserialize)]
    struct InteractionsFrame {
        interactions: Interactions,
    }
}
