use derive_new::new;
use object_model::play::{ChargePoints, HealthPoints, SkillPoints};
use serde::{Deserialize, Serialize};

/// Conditions for a control transition to happen.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ControlTransitionRequirement {
    /// How much `ChargePoints` the object must spend to transition.
    #[serde(default = "ControlTransitionRequirement::charge_default")]
    pub charge: ChargePoints,
    /// How many `HealthPoints` the object must spend to transition.
    #[serde(default = "ControlTransitionRequirement::hp_default")]
    pub hp: HealthPoints,
    /// How many `SkillPoints` the object must spend to transition.
    #[serde(default = "ControlTransitionRequirement::sp_default")]
    pub sp: SkillPoints,
}

impl Default for ControlTransitionRequirement {
    fn default() -> Self {
        ControlTransitionRequirement {
            charge: Self::charge_default(),
            hp: Self::hp_default(),
            sp: Self::sp_default(),
        }
    }
}

impl ControlTransitionRequirement {
    fn charge_default() -> ChargePoints {
        ChargePoints::new(0)
    }

    fn hp_default() -> HealthPoints {
        HealthPoints::new(0)
    }

    fn sp_default() -> SkillPoints {
        SkillPoints::new(0)
    }
}
