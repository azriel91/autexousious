use derivative::Derivative;
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;

use crate::loaded::{ControlTransition, ControlTransitionLike};

/// Sequence transitions upon control input.
#[derive(Clone, Debug, Derivative, Deref, DerefMut, From, PartialEq, Eq, new)]
#[derivative(Default(bound = ""))]
pub struct InputReactions<C = ControlTransition>(pub Vec<C>)
where
    C: ControlTransitionLike + Send + Sync + 'static;
