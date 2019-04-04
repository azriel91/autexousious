use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use specs_derive::Component;

use crate::play::ImpactRepeatTracker;

/// Component that tracks the impact repeat delays for the objects that an entity hits.
#[derive(Component, Clone, Debug, Deref, DerefMut, PartialEq, new)]
pub struct ImpactRepeatTrackers(pub Vec<ImpactRepeatTracker>);
