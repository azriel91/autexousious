use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use fnv::FnvHashMap;
use specs_derive::Component;

use crate::{config::object::SequenceId, loaded::SequenceEndTransition};

/// Component sequence transitions when a sequence ends.
#[derive(Clone, Component, Debug, Default, Deref, DerefMut, From, PartialEq, Eq, new)]
pub struct SequenceEndTransitions<SeqId>(pub FnvHashMap<SeqId, SequenceEndTransition<SeqId>>)
where
    SeqId: SequenceId;
