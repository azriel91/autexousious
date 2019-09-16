pub use self::{
    asset_discovery_system::AssetDiscoverySystem, map_asset_loading_system::MapAssetLoadingSystem,
    object_asset_loading_system::ObjectAssetLoadingSystem,
};

mod asset_discovery_system;
mod map_asset_loading_system;
mod object_asset_loading_system;
