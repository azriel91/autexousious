//! Types representing a background in configuration form.

pub use self::{
    background_definition::{BackgroundDefinition, BackgroundDefinitionHandle},
    layer::Layer,
    layer_frame::LayerFrame,
    layer_position::LayerPosition,
};

mod background_definition;
mod layer;
mod layer_frame;
mod layer_position;
