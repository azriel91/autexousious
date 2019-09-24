use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::SequenceIdMappings;

/// Bi-directional mappings from sequence name to ID for an asset.
pub type AssetSequenceIdMappings<SeqName> = SecondaryMap<AssetId, SequenceIdMappings<SeqName>>;
