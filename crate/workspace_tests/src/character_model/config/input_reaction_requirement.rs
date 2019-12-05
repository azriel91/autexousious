#[cfg(test)]
mod tests {
    use charge_model::{
        config::{ChargePoints, ChargeUseMode},
        play::ChargeTrackerClock,
    };
    use game_input::ControllerInput;
    use game_input_model::config::{InputDirection, InputDirectionZ};
    use object_model::play::{HealthPoints, Mirrored, SkillPoints};

    use character_model::config::{CharacterIrrPart, InputReactionRequirementParams};

    #[test]
    fn health_points_requirement_met_when_greater_equal() {
        let requirement = CharacterIrrPart::Hp(HealthPoints::new(10));
        let params = InputReactionRequirementParams {
            health_points: Some(HealthPoints::new(10)),
            ..Default::default()
        };

        assert!(requirement.is_met(params));

        let params = InputReactionRequirementParams {
            health_points: Some(HealthPoints::new(11)),
            ..Default::default()
        };
        assert!(requirement.is_met(params));
    }

    #[test]
    fn health_points_requirement_not_met_when_less_than() {
        let requirement = CharacterIrrPart::Hp(HealthPoints::new(10));
        let params = InputReactionRequirementParams {
            health_points: Some(HealthPoints::new(9)),
            ..Default::default()
        };

        assert!(!requirement.is_met(params));
    }

    #[test]
    fn skill_points_requirement_met_when_greater_equal() {
        let requirement = CharacterIrrPart::Sp(SkillPoints::new(10));
        let params = InputReactionRequirementParams {
            skill_points: Some(SkillPoints::new(10)),
            ..Default::default()
        };

        assert!(requirement.is_met(params));

        let params = InputReactionRequirementParams {
            skill_points: Some(SkillPoints::new(11)),
            ..Default::default()
        };
        assert!(requirement.is_met(params));
    }

    #[test]
    fn skill_points_requirement_not_met_when_less_than() {
        let requirement = CharacterIrrPart::Sp(SkillPoints::new(10));
        let params = InputReactionRequirementParams {
            skill_points: Some(SkillPoints::new(9)),
            ..Default::default()
        };

        assert!(!requirement.is_met(params));
    }

    #[test]
    fn charge_points_requirement_met_when_greater_equal() {
        let requirement = CharacterIrrPart::Charge(ChargePoints::new(10));
        let params = InputReactionRequirementParams {
            charge_tracker_clock: Some(ChargeTrackerClock::new_with_value(20, 10)),
            charge_use_mode: Some(ChargeUseMode::NearestWhole),
            ..Default::default()
        };

        assert!(requirement.is_met(params));

        let params = InputReactionRequirementParams {
            charge_tracker_clock: Some(ChargeTrackerClock::new_with_value(20, 11)),
            charge_use_mode: Some(ChargeUseMode::NearestWhole),
            ..Default::default()
        };
        assert!(requirement.is_met(params));
    }

    #[test]
    fn charge_points_requirement_not_met_when_less_than() {
        let requirement = CharacterIrrPart::Charge(ChargePoints::new(10));
        let params = InputReactionRequirementParams {
            charge_tracker_clock: Some(ChargeTrackerClock::new_with_value(20, 9)),
            charge_use_mode: Some(ChargeUseMode::NearestWhole),
            ..Default::default()
        };

        assert!(!requirement.is_met(params));
    }

    macro_rules! input_x_test {
        ($test_name:ident, $variant:ident, $controller_input:expr, $mirrored:expr, true $(,)?) => {
            #[test]
            fn $test_name() {
                let requirement = CharacterIrrPart::InputDirX(InputDirection::$variant);

                let params = InputReactionRequirementParams {
                    controller_input: $controller_input,
                    mirrored: $mirrored,
                    ..Default::default()
                };

                assert!(requirement.is_met(params));
            }
        };

        ($test_name:ident, $variant:ident, $controller_input:expr, $mirrored:expr, false $(,)?) => {
            #[test]
            fn $test_name() {
                let requirement = CharacterIrrPart::InputDirX(InputDirection::$variant);

                let params = InputReactionRequirementParams {
                    controller_input: $controller_input,
                    mirrored: $mirrored,
                    ..Default::default()
                };

                assert!(!requirement.is_met(params));
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
                let requirement = CharacterIrrPart::InputDirZ(InputDirectionZ::$variant);

                let params = InputReactionRequirementParams {
                    controller_input: $controller_input,
                    ..Default::default()
                };

                assert!(requirement.is_met(params));
            }
        };

        ($test_name:ident, $variant:ident, $controller_input:expr, false $(,)?) => {
            #[test]
            fn $test_name() {
                let requirement = CharacterIrrPart::InputDirZ(InputDirectionZ::$variant);

                let params = InputReactionRequirementParams {
                    controller_input: $controller_input,
                    ..Default::default()
                };

                assert!(!requirement.is_met(params));
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
