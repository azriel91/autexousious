//! Contains the types that represent processed configuration.

pub use self::{
    component_data_ext::ComponentDataExt, frame_component_data::FrameComponentData,
    sequence_component_data::SequenceComponentData,
};

mod component_data_ext;
mod frame_component_data;
mod sequence_component_data;
