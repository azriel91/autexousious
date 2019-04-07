//! Contains component types for entities.

pub use self::{
    frame_freeze_clock::FrameFreezeClock, frame_index_clock::FrameIndexClock,
    frame_wait_clock::FrameWaitClock, sequence_status::SequenceStatus,
};

mod frame_freeze_clock;
mod frame_index_clock;
mod frame_wait_clock;
mod sequence_status;