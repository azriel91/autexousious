use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::loaded::Spawn;

/// Objects to spawn.
#[derive(Asset, Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct Spawns(pub Vec<Spawn>);
