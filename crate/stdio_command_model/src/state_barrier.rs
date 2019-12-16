use derive_new::new;
use serde::{Deserialize, Serialize};
use state_registry::StateId;
use structopt::StructOpt;

/// Parameters to set up a command barrier.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub struct StateBarrier {
    /// State ID to wait for before commands continue to be issued from stdin.
    pub state_id: StateId,
}
