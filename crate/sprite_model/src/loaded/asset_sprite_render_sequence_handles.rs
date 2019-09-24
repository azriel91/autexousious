use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::SpriteRenderSequenceHandles;

/// Sequence of `SpriteRenderSequenceHandle`s for an asset.
pub type AssetSpriteRenderSequenceHandles = SecondaryMap<AssetId, SpriteRenderSequenceHandles>;
