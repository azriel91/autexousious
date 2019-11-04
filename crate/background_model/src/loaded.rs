//! Types representing loaded background configuration.

pub use self::{
    asset_background_definition_handle::AssetBackgroundDefinitionHandle,
    asset_background_layers::AssetBackgroundLayers, background_layer::BackgroundLayer,
    background_layers::BackgroundLayers,
};

mod asset_background_definition_handle;
mod asset_background_layers;
mod background_layer;
mod background_layers;
