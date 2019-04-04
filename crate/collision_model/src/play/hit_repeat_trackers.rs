use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use specs_derive::Component;

use crate::play::HitRepeatTracker;

/// Component that tracks the hit repeat delays for the objects that an entity hits.
#[derive(Component, Clone, Debug, Deref, DerefMut, PartialEq, new)]
pub struct HitRepeatTrackers(pub Vec<HitRepeatTracker>);
