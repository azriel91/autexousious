use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::config::{ControlArgs, ControllerId};

/// Parameters to the mapper.
///
/// # Examples
///
/// * `control_input 0 axis x -1.0`
/// * `control_input 0 action attack true`
/// * `control_input 0 action attack false`
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, StructOpt)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub struct ControlInputEventArgs {
    /// ID of the controller, as laid out in `input_config.ron`.
    pub controller_id: ControllerId,
    /// Axis or Action
    #[structopt(subcommand)]
    pub control: ControlArgs,
}
