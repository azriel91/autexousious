use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::StateBarrier;

/// Event to control the behaviour of stdio.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, StructOpt)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub enum StdioCommandEvent {
    /// Indicates which `State` must be running before commands continue to be issued.
    StateBarrier(StateBarrier),
}
