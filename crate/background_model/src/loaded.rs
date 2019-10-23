//! Types representing loaded background configuration.

pub use self::{
    asset_background_definition_handle::AssetBackgroundDefinitionHandle,
    asset_layer_positions::AssetLayerPositions, layer_positions::LayerPositions,
};

mod asset_background_definition_handle;
mod asset_layer_positions;
mod layer_positions;
