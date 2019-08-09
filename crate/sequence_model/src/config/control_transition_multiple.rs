use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{ControlTransitionSingle, SequenceId};

/// Configuration type for transition sequence ID.
#[derive(Clone, Debug, Deref, DerefMut, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ControlTransitionMultiple<SeqId, Req>(pub Vec<ControlTransitionSingle<SeqId, Req>>)
where
    SeqId: SequenceId,
    Req: Default;
