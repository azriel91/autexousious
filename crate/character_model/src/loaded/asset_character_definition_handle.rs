use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::config::CharacterDefinitionHandle;

/// `CharacterDefinitionHandle` for an asset.
pub type AssetCharacterDefinitionHandle = SecondaryMap<AssetId, CharacterDefinitionHandle>;
