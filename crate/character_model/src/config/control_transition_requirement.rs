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

    /// Returns whether this is a blank requirement.
    ///
    /// A blank requirement is one that doesn't require anything to be met.
    pub fn is_blank(self) -> bool {
        self.charge == 0 && self.hp == 0 && self.sp == 0
    }
}

#[cfg(test)]
mod tests {
    use object_model::play::{ChargePoints, HealthPoints, SkillPoints};

    use super::ControlTransitionRequirement;

    #[test]
    fn default_all_fields_zero() {
        let requirement = ControlTransitionRequirement::default();
        assert_eq!(ChargePoints::new(0), requirement.charge);
        assert_eq!(HealthPoints::new(0), requirement.hp);
        assert_eq!(SkillPoints::new(0), requirement.sp);
    }

    #[test]
    fn is_blank_returns_true_when_all_fields_zero() {
        assert!(ControlTransitionRequirement::default().is_blank());
    }

    #[test]
    fn is_blank_returns_false_when_charge_non_zero() {
        let requirement = ControlTransitionRequirement {
            charge: ChargePoints::new(10),
            ..Default::default()
        };
        assert!(!requirement.is_blank());
    }

    #[test]
    fn is_blank_returns_false_when_hp_non_zero() {
        let requirement = ControlTransitionRequirement {
            hp: HealthPoints::new(10),
            ..Default::default()
        };
        assert!(!requirement.is_blank());
    }

    #[test]
    fn is_blank_returns_false_when_sp_non_zero() {
        let requirement = ControlTransitionRequirement {
            sp: SkillPoints::new(10),
            ..Default::default()
        };
        assert!(!requirement.is_blank());
    }
}
