use derive_deref::{Deref, DerefMut};
use derive_new::new;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};

use crate::config::ButtonInputReaction;

/// Reactions when device buttons are pressed (`Vec<ButtonInputReaction<SeqName,
/// IRR>>` newtype).
#[derive(Clone, Debug, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ButtonInputReactions<SeqName, IRR>(pub Vec<ButtonInputReaction<SeqName, IRR>>)
where
    SeqName: SequenceName,
    IRR: Default;
