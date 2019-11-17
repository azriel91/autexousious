//! Types that represent processed configuration.

pub use self::{
    asset_object_acceleration_sequence_handles::AssetObjectAccelerationSequenceHandles,
    asset_position_inits::AssetPositionInits,
    object_acceleration_sequence::{ObjectAccelerationSequence, ObjectAccelerationSequenceHandle},
    object_acceleration_sequence_handles::ObjectAccelerationSequenceHandles,
    position_inits::PositionInits,
};

mod asset_object_acceleration_sequence_handles;
mod asset_position_inits;
mod object_acceleration_sequence;
mod object_acceleration_sequence_handles;
mod position_inits;
