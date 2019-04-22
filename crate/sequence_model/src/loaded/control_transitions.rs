use std::marker::PhantomData;

use derivative::Derivative;
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;

use crate::{
    config::SequenceId,
    loaded::{ControlTransition, ControlTransitionLike},
};

/// Sequence transitions upon control input.
#[derive(Clone, Debug, Derivative, Deref, DerefMut, From, PartialEq, Eq, new)]
#[derivative(Default(bound = ""))]
pub struct ControlTransitions<SeqId, C = ControlTransition<SeqId>>(pub Vec<C>, PhantomData<SeqId>)
where
    C: ControlTransitionLike<SeqId> + Send + Sync + 'static,
    SeqId: SequenceId;
