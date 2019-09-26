//! Types that represent processed configuration.

pub use self::{
    asset_spawns_sequence_handles::AssetSpawnsSequenceHandles,
    spawn::Spawn,
    spawns::{Spawns, SpawnsHandle},
    spawns_sequence::{SpawnsSequence, SpawnsSequenceHandle},
    spawns_sequence_handles::SpawnsSequenceHandles,
};

mod asset_spawns_sequence_handles;
mod spawn;
mod spawns;
mod spawns_sequence;
mod spawns_sequence_handles;
