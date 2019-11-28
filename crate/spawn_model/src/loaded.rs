//! Types that represent processed configuration.

pub use self::{
    spawn::Spawn,
    spawns::{Spawns, SpawnsHandle},
    spawns_sequence::{SpawnsSequence, SpawnsSequenceHandle},
    spawns_sequence_handles::SpawnsSequenceHandles,
};

mod spawn;
mod spawns;
mod spawns_sequence;
mod spawns_sequence_handles;
