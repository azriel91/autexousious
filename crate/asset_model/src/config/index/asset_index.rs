use std::collections::HashMap;

use object_type::ObjectType;

use crate::config::index::AssetRecord;

/// Index of all assets.
#[derive(Debug, Default, PartialEq)]
pub struct AssetIndex {
    /// List of objects in the assets directories.
    pub objects: HashMap<ObjectType, Vec<AssetRecord>>,
    /// List of maps in the assets directories
    pub maps: Vec<AssetRecord>,
}
