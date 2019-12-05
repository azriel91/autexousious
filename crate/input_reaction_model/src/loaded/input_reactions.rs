use asset_derive::Asset;
use derivative::Derivative;
use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::loaded::InputReaction;

/// Sequence transitions upon control input.
#[derive(Asset, Clone, Debug, Derivative, Deref, DerefMut, PartialEq, Eq, new)]
#[derivative(Default(bound = ""))]
pub struct InputReactions<IR = InputReaction>(pub Vec<IR>)
where
    IR: Send + Sync + 'static;
