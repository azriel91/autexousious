use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::Spawn;

/// Objects to spawn.
#[derive(Asset, Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct Spawns(pub Vec<Spawn>);
