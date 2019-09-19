use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::SequenceEndTransitions;

/// Sequence transition upon sequence end for an asset.
pub type AssetSequenceEndTransitions = SecondaryMap<AssetId, SequenceEndTransitions>;
