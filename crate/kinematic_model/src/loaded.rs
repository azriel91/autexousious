//! Types that represent processed configuration.

pub use self::{
    object_acceleration_sequence::{ObjectAccelerationSequence, ObjectAccelerationSequenceHandle},
    object_acceleration_sequence_handles::ObjectAccelerationSequenceHandles,
};

mod object_acceleration_sequence;
mod object_acceleration_sequence_handles;
