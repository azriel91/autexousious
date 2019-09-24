//! Types that represent processed configuration.

pub use self::{
    asset_spawns_sequence_handles::AssetSpawnsSequenceHandles,
    spawns_sequence::{SpawnsSequence, SpawnsSequenceHandle},
    spawns_sequence_handles::SpawnsSequenceHandles,
};

mod asset_spawns_sequence_handles;
mod spawns_sequence;
mod spawns_sequence_handles;
