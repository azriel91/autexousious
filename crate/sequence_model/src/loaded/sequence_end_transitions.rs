use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;
use fnv::FnvHashMap;
use specs_derive::Component;

use crate::config::{SequenceEndTransition, SequenceId};

/// Component sequence transitions when a sequence ends.
#[derive(Clone, Component, Debug, Default, Deref, DerefMut, From, PartialEq, new)]
pub struct SequenceEndTransitions<SeqId>(pub FnvHashMap<SeqId, SequenceEndTransition<SeqId>>)
where
    SeqId: SequenceId;
