use object_model::play::{ChargePoints, HealthPoints, SkillPoints};
use serde::{Deserialize, Serialize};

/// Conditions for a control transition to happen.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ControlTransitionRequirement {
    /// How many `ChargePoints` the object must spend to transition.
    Charge(ChargePoints),
    /// How many `HealthPoints` the object must spend to transition.
    Hp(HealthPoints),
    /// How many `SkillPoints` the object must spend to transition.
    Sp(SkillPoints),
}

impl ControlTransitionRequirement {
    /// Returns whether this requirement is met.
    pub fn is_met(
        self,
        health_points: HealthPoints,
        skill_points: SkillPoints,
        charge_points: ChargePoints,
    ) -> bool {
        match self {
            ControlTransitionRequirement::Hp(required) => health_points >= required,
            ControlTransitionRequirement::Sp(required) => skill_points >= required,
            ControlTransitionRequirement::Charge(required) => charge_points >= required,
        }
    }
}

#[cfg(test)]
mod tests {
    use object_model::play::{ChargePoints, HealthPoints, SkillPoints};

    use super::ControlTransitionRequirement;

    #[test]
    fn health_points_meets_requirement_when_greater_equal() {
        let requirement = ControlTransitionRequirement::Hp(HealthPoints::new(10));
        let health_points = HealthPoints::new(10);
        let skill_points = SkillPoints::new(0);
        let charge_points = ChargePoints::new(0);

        assert!(requirement.is_met(health_points, skill_points, charge_points));

        let health_points = HealthPoints::new(11);
        assert!(requirement.is_met(health_points, skill_points, charge_points));
    }

    #[test]
    fn health_points_does_not_meet_requirement_when_less_than() {
        let requirement = ControlTransitionRequirement::Hp(HealthPoints::new(10));
        let health_points = HealthPoints::new(9);
        let skill_points = SkillPoints::new(0);
        let charge_points = ChargePoints::new(0);

        assert!(!requirement.is_met(health_points, skill_points, charge_points));
    }

    #[test]
    fn skill_points_meets_requirement_when_greater_equal() {
        let requirement = ControlTransitionRequirement::Sp(SkillPoints::new(10));
        let health_points = HealthPoints::new(0);
        let skill_points = SkillPoints::new(10);
        let charge_points = ChargePoints::new(0);

        assert!(requirement.is_met(health_points, skill_points, charge_points));

        let skill_points = SkillPoints::new(11);
        assert!(requirement.is_met(health_points, skill_points, charge_points));
    }

    #[test]
    fn skill_points_does_not_meet_requirement_when_less_than() {
        let requirement = ControlTransitionRequirement::Sp(SkillPoints::new(10));
        let health_points = HealthPoints::new(0);
        let skill_points = SkillPoints::new(9);
        let charge_points = ChargePoints::new(0);

        assert!(!requirement.is_met(health_points, skill_points, charge_points));
    }

    #[test]
    fn charge_points_meets_requirement_when_greater_equal() {
        let requirement = ControlTransitionRequirement::Charge(ChargePoints::new(10));
        let health_points = HealthPoints::new(0);
        let skill_points = SkillPoints::new(0);
        let charge_points = ChargePoints::new(10);

        assert!(requirement.is_met(health_points, skill_points, charge_points));

        let charge_points = ChargePoints::new(11);
        assert!(requirement.is_met(health_points, skill_points, charge_points));
    }

    #[test]
    fn charge_points_does_not_meet_requirement_when_less_than() {
        let requirement = ControlTransitionRequirement::Charge(ChargePoints::new(10));
        let health_points = HealthPoints::new(0);
        let skill_points = SkillPoints::new(0);
        let charge_points = ChargePoints::new(9);

        assert!(!requirement.is_met(health_points, skill_points, charge_points));
    }
}
