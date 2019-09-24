use asset_model::loaded::AssetId;
use slotmap::SparseSecondaryMap;

use crate::loaded::Margins;

/// `Margin`s for an asset.
pub type AssetMargins = SparseSecondaryMap<AssetId, Margins>;
