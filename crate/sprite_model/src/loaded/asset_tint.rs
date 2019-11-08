use amethyst::renderer::resources::Tint;
use asset_model::loaded::AssetId;
use slotmap::SparseSecondaryMap;

/// `Tint`s for an asset.
pub type AssetTint = SparseSecondaryMap<AssetId, Tint>;
