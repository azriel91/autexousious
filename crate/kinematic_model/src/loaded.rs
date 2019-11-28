//! Types that represent processed configuration.

pub use self::{
    object_acceleration_sequence::{ObjectAccelerationSequence, ObjectAccelerationSequenceHandle},
    object_acceleration_sequence_handles::ObjectAccelerationSequenceHandles,
    position_inits::PositionInits,
};

mod object_acceleration_sequence;
mod object_acceleration_sequence_handles;
mod position_inits;
