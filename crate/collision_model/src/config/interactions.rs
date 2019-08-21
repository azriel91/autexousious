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
    use object_status_model::config::StunPoints;
    use serde::Deserialize;
    use serde_yaml;
    use shape_model::Volume;

    use super::Interactions;
    use crate::config::{Hit, HitLimit, HitRepeatDelay, Interaction, InteractionKind};

    const ITR_PHYSICAL_ALL_SPECIFIED: &str = r#"---
interactions:
  - hit: { repeat_delay: 5, hit_limit: "unlimited", hp_damage: 40, sp_damage: 50, stun: 33 }
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
                repeat_delay: HitRepeatDelay::default(),
                hit_limit: HitLimit::Limit(2),
                hp_damage: 0,
                sp_damage: 0,
                stun: StunPoints::default(),
            }),
            multiple: Default::default(),
        }]; // kcov-ignore
        assert_eq!(Interactions::new(interactions), frame.interactions);
    }

    /// Needed because the YAML deserializer does not support deserializing values directly.
    #[derive(Debug, Deserialize)]
    struct InteractionsFrame {
        interactions: Interactions,
    }
}
