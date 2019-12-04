use derive_new::new;
use sequence_model::config::{SequenceName, SequenceNameString};
use serde::{Deserialize, Serialize};

/// Configuration type for transition sequence name.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct InputReactionSingle<SeqName, Req>
where
    SeqName: SequenceName,
{
    /// Sequence name to transition to.
    pub next: SequenceNameString<SeqName>,
    /// Additional requirements for the `InputReaction`.
    #[serde(default)]
    pub requirements: Req,
}
