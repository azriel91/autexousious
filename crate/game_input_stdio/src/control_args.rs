use game_input_model::{Axis, ControlAction};
use structopt_derive::StructOpt;
use strum_macros::EnumString;

/// Parameters for control input.
#[derive(Clone, Copy, Debug, EnumString, PartialEq, StructOpt)]
#[strum(serialize_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub enum ControlArgs {
    /// Axis input.
    Axis {
        /// Axis to control, "x" or "z".
        axis: Axis,
        /// Value to use for the axis input.
        value: f32,
    },
    /// Action pressed.
    ActionPressed {
        /// Action to control, "defend", "jump", "attack", "special".
        action: ControlAction,
    },
    /// Action released.
    ActionReleased {
        /// Action to control, "defend", "jump", "attack", "special".
        action: ControlAction,
    },
}
