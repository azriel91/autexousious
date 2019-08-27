use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{SequenceName, SequenceNameString};

/// Configuration type for transition sequence name.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ControlTransitionSingle<SeqName, Req>
where
    SeqName: SequenceName,
{
    /// Sequence name to transition to.
    pub next: SequenceNameString<SeqName>,
    /// Additional requirements for the `ControlTransition`.
    #[serde(default)]
    pub requirements: Req,
}
