use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{ControlTransitionSingle, SequenceName};

/// Configuration type for transition sequence name.
#[derive(Clone, Debug, Deref, DerefMut, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ControlTransitionMultiple<SeqName, Req>(pub Vec<ControlTransitionSingle<SeqName, Req>>)
where
    SeqName: SequenceName,
    Req: Default;
