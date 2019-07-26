use game_input::ControllerInput;
use object_model::play::{ChargePoints, HealthPoints, Mirrored, SkillPoints};
use serde::{Deserialize, Serialize};

/// Conditions for a control transition to happen.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ControlTransitionRequirement {
    /// `ChargePoints` the object must spend to transition.
    Charge(ChargePoints),
    /// `HealthPoints` the object must spend to transition.
    Hp(HealthPoints),
    /// `SkillPoints` the object must spend to transition.
    Sp(SkillPoints),
    /// Whether there is axis input, and if it matches the direction the object is facing.
    InputDirection(bool),
}

impl ControlTransitionRequirement {
    /// Returns whether this requirement is met.
    pub fn is_met(
        self,
        health_points: Option<HealthPoints>,
        skill_points: Option<SkillPoints>,
        charge_points: Option<ChargePoints>,
        mirrored: Option<Mirrored>,
        controller_input: Option<ControllerInput>,
    ) -> bool {
        match self {
            ControlTransitionRequirement::Hp(required) => {
                health_points.map(|points| points >= required)
            }
            ControlTransitionRequirement::Sp(required) => {
                skill_points.map(|points| points >= required)
            }
            ControlTransitionRequirement::Charge(required) => {
                charge_points.map(|points| points >= required)
            }
            ControlTransitionRequirement::InputDirection(match_direction) => {
                if let (Some(mirrored), Some(controller_input)) = (mirrored, controller_input) {
                    let input_matches_direction = controller_input.x_axis_value > 0. && !mirrored.0
                        || controller_input.x_axis_value < 0. && mirrored.0;
                    let requirement_met = match_direction == input_matches_direction;
                    Some(requirement_met)
                } else {
                    Some(false)
                }
            }
        }
        .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::play::{ChargePoints, HealthPoints, Mirrored, SkillPoints};

    use super::ControlTransitionRequirement;

    #[test]
    fn health_points_meets_requirement_when_greater_equal() {
        let requirement = ControlTransitionRequirement::Hp(HealthPoints::new(10));
        let health_points = Some(HealthPoints::new(10));

        assert!(requirement.is_met(health_points, None, None, None, None));

        let health_points = Some(HealthPoints::new(11));
        assert!(requirement.is_met(health_points, None, None, None, None));
    }

    #[test]
    fn health_points_does_not_meet_requirement_when_less_than() {
        let requirement = ControlTransitionRequirement::Hp(HealthPoints::new(10));
        let health_points = Some(HealthPoints::new(9));

        assert!(!requirement.is_met(health_points, None, None, None, None));
    }

    #[test]
    fn skill_points_meets_requirement_when_greater_equal() {
        let requirement = ControlTransitionRequirement::Sp(SkillPoints::new(10));
        let skill_points = Some(SkillPoints::new(10));

        assert!(requirement.is_met(None, skill_points, None, None, None));

        let skill_points = Some(SkillPoints::new(11));
        assert!(requirement.is_met(None, skill_points, None, None, None));
    }

    #[test]
    fn skill_points_does_not_meet_requirement_when_less_than() {
        let requirement = ControlTransitionRequirement::Sp(SkillPoints::new(10));
        let skill_points = Some(SkillPoints::new(9));

        assert!(!requirement.is_met(None, skill_points, None, None, None));
    }

    #[test]
    fn charge_points_meets_requirement_when_greater_equal() {
        let requirement = ControlTransitionRequirement::Charge(ChargePoints::new(10));
        let charge_points = Some(ChargePoints::new(10));

        assert!(requirement.is_met(None, None, charge_points, None, None));

        let charge_points = Some(ChargePoints::new(11));
        assert!(requirement.is_met(None, None, charge_points, None, None));
    }

    #[test]
    fn charge_points_does_not_meet_requirement_when_less_than() {
        let requirement = ControlTransitionRequirement::Charge(ChargePoints::new(10));
        let charge_points = Some(ChargePoints::new(9));

        assert!(!requirement.is_met(None, None, charge_points, None, None));
    }

    #[test]
    fn input_direction_meets_requirement_when_input_matches_match_direction() {
        let requirement = ControlTransitionRequirement::InputDirection(true);

        let mirrored = Some(Mirrored::new(false));
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;
        let controller_input = Some(controller_input);

        assert!(requirement.is_met(None, None, None, mirrored, controller_input));

        let mirrored = Some(Mirrored::new(true));
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = -1.;
        let controller_input = Some(controller_input);

        assert!(requirement.is_met(None, None, None, mirrored, controller_input));
    }

    #[test]
    fn input_direction_does_not_meet_requirement_when_input_does_not_match_match_direction() {
        let requirement = ControlTransitionRequirement::InputDirection(true);

        let mirrored = Some(Mirrored::new(false));
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = -1.;
        let controller_input = Some(controller_input);

        assert!(!requirement.is_met(None, None, None, mirrored, controller_input));

        let mirrored = Some(Mirrored::new(true));
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;
        let controller_input = Some(controller_input);

        assert!(!requirement.is_met(None, None, None, mirrored, controller_input));

        // Test zero case.
        let mirrored = Some(Mirrored::new(true));
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 0.;
        let controller_input = Some(controller_input);

        assert!(!requirement.is_met(None, None, None, mirrored, controller_input));

        let mirrored = Some(Mirrored::new(false));
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 0.;
        let controller_input = Some(controller_input);

        assert!(!requirement.is_met(None, None, None, mirrored, controller_input));
    }
}
