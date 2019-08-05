use approx::{relative_eq, relative_ne};
use charge_model::config::ChargePoints;
use game_input::ControllerInput;
use game_input_model::config::{InputDirection, InputDirectionZ};
use object_model::play::{HealthPoints, Mirrored, SkillPoints};
use serde::{Deserialize, Serialize};

/// Conditions for a control transition to happen.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ControlTransitionRequirement {
    /// `ChargePoints` the object must spend to transition.
    Charge(ChargePoints),
    /// `HealthPoints` the object must spend to transition.
    Hp(HealthPoints),
    /// `SkillPoints` the object must spend to transition.
    Sp(SkillPoints),
    /// Whether or not there is x axis input, and if it matches the direction the object is facing.
    InputDirX(InputDirection),
    /// Whether or not there is z axis input, and the direction it is in.
    InputDirZ(InputDirectionZ),
}

impl ControlTransitionRequirement {
    /// Returns whether this requirement is met.
    pub fn is_met(
        self,
        health_points: Option<HealthPoints>,
        skill_points: Option<SkillPoints>,
        charge_points: Option<ChargePoints>,
        controller_input: Option<ControllerInput>,
        mirrored: Option<Mirrored>,
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
            ControlTransitionRequirement::InputDirX(input_direction) => {
                let requirement_met =
                    Self::input_requirement_met_x(controller_input, mirrored, input_direction);
                Some(requirement_met)
            }
            ControlTransitionRequirement::InputDirZ(input_direction_z) => {
                let requirement_met =
                    Self::input_requirement_met_z(controller_input, input_direction_z);
                Some(requirement_met)
            }
        }
        .unwrap_or(false)
    }

    fn input_requirement_met_x(
        controller_input: Option<ControllerInput>,
        mirrored: Option<Mirrored>,
        input_direction: InputDirection,
    ) -> bool {
        match input_direction {
            InputDirection::None => {
                if let Some(controller_input) = controller_input {
                    relative_eq!(0., controller_input.x_axis_value)
                } else {
                    false
                }
            }
            InputDirection::Same => {
                if let (Some(controller_input), Some(mirrored)) = (controller_input, mirrored) {
                    InputDirection::input_matches_direction(
                        controller_input.x_axis_value,
                        *mirrored,
                    )
                } else {
                    false
                }
            }
            InputDirection::Mirrored => {
                if let (Some(controller_input), Some(mirrored)) = (controller_input, mirrored) {
                    InputDirection::input_opposes_direction(
                        controller_input.x_axis_value,
                        *mirrored,
                    )
                } else {
                    false
                }
            }
            InputDirection::Some => {
                if let Some(controller_input) = controller_input {
                    relative_ne!(0., controller_input.x_axis_value)
                } else {
                    false
                }
            }
            InputDirection::NotSame => {
                if let Some(controller_input) = controller_input {
                    relative_eq!(0., controller_input.x_axis_value)
                        || if let Some(mirrored) = mirrored {
                            InputDirection::input_opposes_direction(
                                controller_input.x_axis_value,
                                *mirrored,
                            )
                        } else {
                            false
                        }
                } else {
                    false
                }
            }
            InputDirection::NotMirrored => {
                if let Some(controller_input) = controller_input {
                    relative_eq!(0., controller_input.x_axis_value)
                        || if let Some(mirrored) = mirrored {
                            InputDirection::input_matches_direction(
                                controller_input.x_axis_value,
                                *mirrored,
                            )
                        } else {
                            false
                        }
                } else {
                    false
                }
            }
        }
    }

    fn input_requirement_met_z(
        controller_input: Option<ControllerInput>,
        input_direction_z: InputDirectionZ,
    ) -> bool {
        if controller_input.is_none() {
            return false;
        }
        let controller_input = controller_input.expect("Unreachable: Already returned if none.");
        match input_direction_z {
            InputDirectionZ::None => relative_eq!(0., controller_input.z_axis_value),
            InputDirectionZ::Up => controller_input.z_axis_value < 0.,
            InputDirectionZ::Down => controller_input.z_axis_value > 0.,
            InputDirectionZ::Some => relative_ne!(0., controller_input.z_axis_value),
            InputDirectionZ::NotUp => {
                relative_eq!(0., controller_input.z_axis_value)
                    || controller_input.z_axis_value > 0.
            }
            InputDirectionZ::NotDown => {
                relative_eq!(0., controller_input.z_axis_value)
                    || controller_input.z_axis_value < 0.
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use charge_model::config::ChargePoints;
    use game_input::ControllerInput;
    use game_input_model::config::{InputDirection, InputDirectionZ};
    use object_model::play::{HealthPoints, Mirrored, SkillPoints};

    use super::ControlTransitionRequirement;

    #[test]
    fn health_points_requirement_met_when_greater_equal() {
        let requirement = ControlTransitionRequirement::Hp(HealthPoints::new(10));
        let health_points = Some(HealthPoints::new(10));

        assert!(requirement.is_met(health_points, None, None, None, None));

        let health_points = Some(HealthPoints::new(11));
        assert!(requirement.is_met(health_points, None, None, None, None));
    }

    #[test]
    fn health_points_requirement_not_met_when_less_than() {
        let requirement = ControlTransitionRequirement::Hp(HealthPoints::new(10));
        let health_points = Some(HealthPoints::new(9));

        assert!(!requirement.is_met(health_points, None, None, None, None));
    }

    #[test]
    fn skill_points_requirement_met_when_greater_equal() {
        let requirement = ControlTransitionRequirement::Sp(SkillPoints::new(10));
        let skill_points = Some(SkillPoints::new(10));

        assert!(requirement.is_met(None, skill_points, None, None, None));

        let skill_points = Some(SkillPoints::new(11));
        assert!(requirement.is_met(None, skill_points, None, None, None));
    }

    #[test]
    fn skill_points_requirement_not_met_when_less_than() {
        let requirement = ControlTransitionRequirement::Sp(SkillPoints::new(10));
        let skill_points = Some(SkillPoints::new(9));

        assert!(!requirement.is_met(None, skill_points, None, None, None));
    }

    #[test]
    fn charge_points_requirement_met_when_greater_equal() {
        let requirement = ControlTransitionRequirement::Charge(ChargePoints::new(10));
        let charge_points = Some(ChargePoints::new(10));

        assert!(requirement.is_met(None, None, charge_points, None, None));

        let charge_points = Some(ChargePoints::new(11));
        assert!(requirement.is_met(None, None, charge_points, None, None));
    }

    #[test]
    fn charge_points_requirement_not_met_when_less_than() {
        let requirement = ControlTransitionRequirement::Charge(ChargePoints::new(10));
        let charge_points = Some(ChargePoints::new(9));

        assert!(!requirement.is_met(None, None, charge_points, None, None));
    }

    macro_rules! input_x_test {
        ($test_name:ident, $variant:ident, $controller_input:expr, $mirrored:expr, true $(,)?) => {
            #[test]
            fn $test_name() {
                let requirement = ControlTransitionRequirement::InputDirX(InputDirection::$variant);

                let controller_input = $controller_input;
                let mirrored = $mirrored;

                assert!(requirement.is_met(None, None, None, controller_input, mirrored));
            }
        };

        ($test_name:ident, $variant:ident, $controller_input:expr, $mirrored:expr, false $(,)?) => {
            #[test]
            fn $test_name() {
                let requirement = ControlTransitionRequirement::InputDirX(InputDirection::$variant);

                let controller_input = $controller_input;
                let mirrored = $mirrored;

                assert!(!requirement.is_met(None, None, None, controller_input, mirrored));
            }
        };
    }

    // None variant
    input_x_test!(
        input_x_requirement_met_when_requirement_none_and_input_zero,
        None,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        None,
        true,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_none_and_input_positive,
        None,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        None,
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_none_and_input_negative,
        None,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        None,
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_none_and_no_controller_input,
        None,
        None,
        None,
        false,
    );

    // Same variant
    input_x_test!(
        input_x_requirement_met_when_requirement_same_and_input_matches_direction,
        Same,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        true,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_same_and_input_matches_direction_mirrored,
        Same,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        true,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_same_and_input_opposes_direction,
        Same,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_same_and_input_opposes_direction_mirrored,
        Same,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_same_and_input_zero,
        Same,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_same_and_input_zero_mirrored,
        Same,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_same_and_no_controller_input,
        Same,
        None,
        Some(Mirrored::new(true)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_same_and_no_mirrored_component,
        Same,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        None,
        false,
    );

    // Mirrored variant
    input_x_test!(
        input_x_requirement_not_met_when_requirement_mirrored_and_input_matches_direction,
        Mirrored,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_mirrored_and_input_matches_direction_mirrored,
        Mirrored,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        false,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_mirrored_and_input_opposes_direction,
        Mirrored,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        true,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_mirrored_and_input_opposes_direction_mirrored,
        Mirrored,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        true,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_mirrored_and_input_zero,
        Mirrored,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_mirrored_and_input_zero_mirrored,
        Mirrored,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_mirrored_and_no_controller_input,
        Mirrored,
        None,
        Some(Mirrored::new(true)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_mirrored_and_no_mirrored_component,
        Mirrored,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        None,
        false,
    );

    // Some variant
    input_x_test!(
        input_x_requirement_not_met_when_requirement_some_and_input_zero,
        Some,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        None,
        false,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_some_and_input_positive,
        Some,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        None,
        true,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_some_and_input_negative,
        Some,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        None,
        true,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_some_and_no_controller_input,
        Some,
        None,
        None,
        false,
    );

    // NotSame variant
    input_x_test!(
        input_x_requirement_not_met_when_requirement_not_same_and_input_matches_direction,
        NotSame,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_not_same_and_input_matches_direction_mirrored,
        NotSame,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        false,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_not_same_and_input_opposes_direction,
        NotSame,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        true,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_not_same_and_input_opposes_direction_mirrored,
        NotSame,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        true,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_not_same_and_input_zero,
        NotSame,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        true,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_not_same_and_input_zero_mirrored,
        NotSame,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        true,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_not_same_and_no_controller_input,
        NotSame,
        None,
        Some(Mirrored::new(true)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_not_same_and_no_mirrored_component,
        NotSame,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        None,
        false,
    );

    // NotMirrored variant
    input_x_test!(
        input_x_requirement_met_when_requirement_not_mirrored_and_input_matches_direction,
        NotMirrored,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        true,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_not_mirrored_and_input_matches_direction_mirrored,
        NotMirrored,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        true,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_not_mirrored_and_input_opposes_direction,
        NotMirrored,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_not_mirrored_and_input_opposes_direction_mirrored,
        NotMirrored,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        false,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_not_mirrored_and_input_zero,
        NotMirrored,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        Some(Mirrored::new(false)),
        true,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_not_mirrored_and_input_zero_mirrored,
        NotMirrored,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        Some(Mirrored::new(true)),
        true,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_not_mirrored_and_no_controller_input,
        NotMirrored,
        None,
        Some(Mirrored::new(true)),
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_not_mirrored_and_no_mirrored_component,
        NotMirrored,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        None,
        false,
    );

    macro_rules! input_z_test {
        ($test_name:ident, $variant:ident, $controller_input:expr, true $(,)?) => {
            #[test]
            fn $test_name() {
                let requirement =
                    ControlTransitionRequirement::InputDirZ(InputDirectionZ::$variant);

                let controller_input = $controller_input;

                assert!(requirement.is_met(None, None, None, controller_input, None));
            }
        };

        ($test_name:ident, $variant:ident, $controller_input:expr, false $(,)?) => {
            #[test]
            fn $test_name() {
                let requirement =
                    ControlTransitionRequirement::InputDirZ(InputDirectionZ::$variant);

                let controller_input = $controller_input;

                assert!(!requirement.is_met(None, None, None, controller_input, None));
            }
        };
    }

    // All variants
    input_z_test!(
        input_z_requirement_not_met_when_no_controller_input,
        NotDown,
        None,
        false,
    );

    // None variant
    input_z_test!(
        input_z_requirement_met_when_requirement_none_and_input_zero,
        None,
        Some(ControllerInput {
            z_axis_value: 0.,
            ..Default::default()
        }),
        true,
    );
    input_z_test!(
        input_z_requirement_not_met_when_requirement_none_and_input_down,
        None,
        Some(ControllerInput {
            z_axis_value: 1.,
            ..Default::default()
        }),
        false,
    );
    input_z_test!(
        input_z_requirement_not_met_when_requirement_none_and_input_up,
        None,
        Some(ControllerInput {
            z_axis_value: -1.,
            ..Default::default()
        }),
        false,
    );

    // Up variant
    input_z_test!(
        input_z_requirement_not_met_when_requirement_up_and_input_down,
        Up,
        Some(ControllerInput {
            z_axis_value: 1.,
            ..Default::default()
        }),
        false,
    );
    input_z_test!(
        input_z_requirement_met_when_requirement_up_and_input_up,
        Up,
        Some(ControllerInput {
            z_axis_value: -1.,
            ..Default::default()
        }),
        true,
    );
    input_z_test!(
        input_z_requirement_not_met_when_requirement_up_and_input_zero,
        Up,
        Some(ControllerInput {
            z_axis_value: 0.,
            ..Default::default()
        }),
        false,
    );

    // Down variant
    input_z_test!(
        input_z_requirement_met_when_requirement_down_and_input_down,
        Down,
        Some(ControllerInput {
            z_axis_value: 1.,
            ..Default::default()
        }),
        true,
    );
    input_z_test!(
        input_z_requirement_not_met_when_requirement_down_and_input_up,
        Down,
        Some(ControllerInput {
            z_axis_value: -1.,
            ..Default::default()
        }),
        false,
    );
    input_z_test!(
        input_z_requirement_not_met_when_requirement_down_and_input_zero,
        Down,
        Some(ControllerInput {
            z_axis_value: 0.,
            ..Default::default()
        }),
        false,
    );

    // Some variant
    input_z_test!(
        input_z_requirement_not_met_when_requirement_some_and_input_zero,
        Some,
        Some(ControllerInput {
            z_axis_value: 0.,
            ..Default::default()
        }),
        false,
    );
    input_z_test!(
        input_z_requirement_met_when_requirement_some_and_input_down,
        Some,
        Some(ControllerInput {
            z_axis_value: 1.,
            ..Default::default()
        }),
        true,
    );
    input_z_test!(
        input_z_requirement_met_when_requirement_some_and_input_up,
        Some,
        Some(ControllerInput {
            z_axis_value: -1.,
            ..Default::default()
        }),
        true,
    );

    // NotUp variant
    input_z_test!(
        input_z_requirement_met_when_requirement_not_up_and_input_down,
        NotUp,
        Some(ControllerInput {
            z_axis_value: 1.,
            ..Default::default()
        }),
        true,
    );
    input_z_test!(
        input_z_requirement_not_met_when_requirement_not_up_and_input_up,
        NotUp,
        Some(ControllerInput {
            z_axis_value: -1.,
            ..Default::default()
        }),
        false,
    );
    input_z_test!(
        input_z_requirement_met_when_requirement_not_up_and_input_zero,
        NotUp,
        Some(ControllerInput {
            z_axis_value: 0.,
            ..Default::default()
        }),
        true,
    );

    // NotDown variant
    input_z_test!(
        input_z_requirement_not_met_when_requirement_not_down_and_input_down,
        NotDown,
        Some(ControllerInput {
            z_axis_value: 1.,
            ..Default::default()
        }),
        false,
    );
    input_z_test!(
        input_z_requirement_met_when_requirement_not_down_and_input_up,
        NotDown,
        Some(ControllerInput {
            z_axis_value: -1.,
            ..Default::default()
        }),
        true,
    );
    input_z_test!(
        input_z_requirement_met_when_requirement_not_down_and_input_zero,
        NotDown,
        Some(ControllerInput {
            z_axis_value: 0.,
            ..Default::default()
        }),
        true,
    );
}
