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
    /// Axis input.
    Action {
        /// Action to control, "defend", "jump", "attack", "special".
        action: ControlAction,
        /// Value to use for the action input.
        #[structopt(parse(try_from_str))] // Treat as a value, not a flag.
        value: bool,
    },
}
