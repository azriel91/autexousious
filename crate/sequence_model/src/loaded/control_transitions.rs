use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;
use specs_derive::Component;

use crate::{config::SequenceId, loaded::ControlTransition};

/// Sequence transitions upon control input.
#[derive(Clone, Component, Debug, Default, Deref, DerefMut, From, PartialEq, Eq, new)]
pub struct ControlTransitions<SeqId>(pub Vec<ControlTransition<SeqId>>)
where
    SeqId: SequenceId;
