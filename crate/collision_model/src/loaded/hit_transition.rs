use amethyst::ecs::{storage::VecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use sequence_model::loaded::SequenceId;

/// Sequence to transition to when hit by another entity.
#[derive(Clone, Component, Copy, Debug, Deref, DerefMut, PartialEq, new)]
#[storage(VecStorage)]
pub struct HitTransition(pub SequenceId);
