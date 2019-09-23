use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::config::EnergyDefinitionHandle;

/// `EnergyDefinitionHandle` for an asset.
pub type AssetEnergyDefinitionHandle = SecondaryMap<AssetId, EnergyDefinitionHandle>;
