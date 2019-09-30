use std::cmp::PartialEq;

use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use slotmap::{DefaultKey, SlotMap};

use crate::play::HitRepeatTracker;

/// Component that tracks the hit repeat delays for the objects that an entity hits.
#[derive(Component, Clone, Debug, Deref, DerefMut, new)]
pub struct HitRepeatTrackers(pub SlotMap<DefaultKey, HitRepeatTracker>);

impl PartialEq for HitRepeatTrackers {
    fn eq(&self, other: &HitRepeatTrackers) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(left, right)| left == right)
    }
}
