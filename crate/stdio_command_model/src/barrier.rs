use derive_new::new;
use state_registry::StateId;
use structopt::StructOpt;

/// Parameters to set up a command barrier.
#[derive(Clone, Copy, Debug, PartialEq, StructOpt, new)]
#[structopt(rename_all = "snake_case")]
pub struct Barrier {
    /// State ID to wait for before commands continue to be issued from stdin.
    pub state_id: StateId,
}
