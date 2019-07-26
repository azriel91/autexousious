use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::SequenceId;

/// Configuration type for transition sequence ID.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ControlTransitionSingle<SeqId, Req>
where
    SeqId: SequenceId,
{
    /// Sequence ID to transition to.
    pub next: SeqId,
    /// Additional requirements for the `ControlTransition`.
    #[serde(default)]
    pub requirements: Req,
}
