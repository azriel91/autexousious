use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::CharacterCtsHandles;

/// `CharacterCtsHandles` for an asset.
pub type AssetCharacterCtsHandles = SecondaryMap<AssetId, CharacterCtsHandles>;
