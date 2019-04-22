use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::SequenceId;

/// Configuration type for transition sequence ID.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ControlTransitionSingle<SeqId, Extra>
where
    SeqId: SequenceId,
{
    /// Sequence ID to transition to.
    pub next: SeqId,
    /// Extra parameters for the `ControlTransition`.
    #[serde(flatten)]
    pub extra: Extra,
}
