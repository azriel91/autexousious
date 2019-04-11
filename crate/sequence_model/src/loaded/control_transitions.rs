use std::marker::PhantomData;

use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;
use specs_derive::Component;

use crate::{
    config::SequenceId,
    loaded::{ControlTransition, ControlTransitionLike},
};

/// Sequence transitions upon control input.
#[derive(Clone, Component, Debug, Default, Deref, DerefMut, From, PartialEq, Eq, new)]
pub struct ControlTransitions<SeqId, C = ControlTransition<SeqId>>(pub Vec<C>, PhantomData<SeqId>)
where
    C: ControlTransitionLike<SeqId> + Send + Sync + 'static,
    SeqId: SequenceId;
