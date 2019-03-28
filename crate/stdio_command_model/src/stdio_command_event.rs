use structopt::StructOpt;

use crate::StateBarrier;

/// Event to control the behaviour of stdio.
#[derive(Clone, Debug, PartialEq, StructOpt)]
#[structopt(rename_all = "snake_case")]
pub enum StdioCommandEvent {
    /// Indicates which `State` must be running before commands continue to be issued.
    StateBarrier(StateBarrier),
}
