//! Types that represent processed configuration.

pub use self::{
    asset_object_acceleration_sequence_handles::AssetObjectAccelerationSequenceHandles,
    object_acceleration_sequence::{ObjectAccelerationSequence, ObjectAccelerationSequenceHandle},
    object_acceleration_sequence_handles::ObjectAccelerationSequenceHandles,
};

mod asset_object_acceleration_sequence_handles;
mod object_acceleration_sequence;
mod object_acceleration_sequence_handles;
