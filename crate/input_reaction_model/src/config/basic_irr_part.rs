use approx::{relative_eq, relative_ne};
use game_input::ControllerInput;
use game_input_model::config::{InputDirection, InputDirectionZ};
use mirrored_model::play::Mirrored;
use serde::{Deserialize, Serialize};

use crate::config::BasicIrrParams;

/// `CharacterInputReactionRequirementPart`
///
/// Conditions for a character input reaction to happen.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum BasicIrrPart {
    /// Whether or not there is x axis input, and if it matches the direction the object is facing.
    InputDirX(InputDirection),
    /// Whether or not there is z axis input, and the direction it is in.
    InputDirZ(InputDirectionZ),
}

impl BasicIrrPart {
    /// Returns whether this requirement is met.
    pub fn is_met(
        self,
        BasicIrrParams {
            controller_input,
            mirrored,
        }: BasicIrrParams,
    ) -> bool {
        match self {
            Self::InputDirX(input_direction) => {
                let requirement_met =
                    Self::input_requirement_met_x(controller_input, mirrored, input_direction);
                Some(requirement_met)
            }
            Self::InputDirZ(input_direction_z) => {
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
