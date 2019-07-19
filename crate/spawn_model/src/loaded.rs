//! Types that represent processed configuration.

pub use self::{
    spawns_sequence::{SpawnsSequence, SpawnsSequenceHandle},
    spawns_sequence_handles::SpawnsSequenceHandles,
};

mod spawns_sequence;
mod spawns_sequence_handles;
