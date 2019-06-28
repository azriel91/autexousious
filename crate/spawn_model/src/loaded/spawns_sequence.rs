use amethyst::assets::Handle;
use sequence_model_derive::component_sequence;

use crate::config::Spawns;

/// Sequence of `Spawn` values.
#[component_sequence(Handle<Spawns>)]
pub struct SpawnsSequence;
