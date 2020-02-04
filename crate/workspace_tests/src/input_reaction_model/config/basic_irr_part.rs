#[cfg(test)]
mod tests {
    use game_input_model::{
        config::{InputDirection, InputDirectionZ},
        play::ControllerInput,
    };
    use mirrored_model::play::Mirrored;

    use input_reaction_model::config::{BasicIrrParams, BasicIrrPart};

    macro_rules! input_x_test {
        ($test_name:ident, $variant:ident, $controller_input:expr, $mirrored:expr, true $(,)?) => {
            #[test]
            fn $test_name() {
                let requirement = BasicIrrPart::InputDirX(InputDirection::$variant);

                let params = BasicIrrParams {
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
                let requirement = BasicIrrPart::InputDirX(InputDirection::$variant);

                let params = BasicIrrParams {
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

    // Left variant
    input_x_test!(
        input_x_requirement_not_met_when_requirement_left_and_input_positive,
        Left,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        None,
        false,
    );
    input_x_test!(
        input_x_requirement_met_when_requirement_left_and_negative,
        Left,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        None,
        true,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_left_and_input_zero,
        Left,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        None,
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_left_and_no_controller_input,
        Left,
        None,
        Some(Mirrored::new(true)),
        false,
    );

    // Right variant
    input_x_test!(
        input_x_requirement_met_when_requirement_right_and_input_positive,
        Right,
        Some(ControllerInput {
            x_axis_value: 1.,
            ..Default::default()
        }),
        None,
        true,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_right_and_negative,
        Right,
        Some(ControllerInput {
            x_axis_value: -1.,
            ..Default::default()
        }),
        None,
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_right_and_input_zero,
        Right,
        Some(ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        }),
        None,
        false,
    );
    input_x_test!(
        input_x_requirement_not_met_when_requirement_right_and_no_controller_input,
        Right,
        None,
        Some(Mirrored::new(true)),
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
                let requirement = BasicIrrPart::InputDirZ(InputDirectionZ::$variant);

                let params = BasicIrrParams {
                    controller_input: $controller_input,
                    ..Default::default()
                };

                assert!(requirement.is_met(params));
            }
        };

        ($test_name:ident, $variant:ident, $controller_input:expr, false $(,)?) => {
            #[test]
            fn $test_name() {
                let requirement = BasicIrrPart::InputDirZ(InputDirectionZ::$variant);

                let params = BasicIrrParams {
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
