use derivative::Derivative;
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;

use crate::loaded::InputReaction;

/// Sequence transitions upon control input.
#[derive(Clone, Debug, Derivative, Deref, DerefMut, From, PartialEq, Eq, new)]
#[derivative(Default(bound = ""))]
pub struct InputReactions<C = InputReaction>(pub Vec<C>)
where
    C: AsRef<InputReaction> + Send + Sync + 'static;
