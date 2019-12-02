use derive_deref::{Deref, DerefMut};
use derive_new::new;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};

use crate::config::InputReactionSingle;

/// Configuration type for transition sequence name.
#[derive(Clone, Debug, Deref, DerefMut, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct InputReactionMultiple<SeqName, Req>(pub Vec<InputReactionSingle<SeqName, Req>>)
where
    SeqName: SequenceName,
    Req: Default;
