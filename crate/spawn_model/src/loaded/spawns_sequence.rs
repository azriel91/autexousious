use amethyst::assets::Handle;
use sequence_model_derive::frame_component_data;

use crate::config::Spawns;

/// Sequence of `Spawn` values.
#[frame_component_data(Handle<Spawns>)]
pub struct SpawnsSequence;
