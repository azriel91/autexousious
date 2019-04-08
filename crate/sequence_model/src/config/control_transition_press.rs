use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::SequenceId;

/// Transition to a specified sequence on control input press event.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, new)]
pub struct ControlTransitionPress<SeqId>
where
    SeqId: SequenceId,
{
    /// ID of the sequence to switch to after this one has completed.
    pub next: SeqId,
}
