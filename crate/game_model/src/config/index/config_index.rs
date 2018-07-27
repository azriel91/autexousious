use std::collections::HashMap;

use object_model::ObjectType;

use config::index::ConfigRecord;

/// Index of all configuration records.
#[derive(Debug, Default, PartialEq)]
pub struct ConfigIndex {
    /// List of objects in the assets directories.
    pub objects: HashMap<ObjectType, Vec<ConfigRecord>>,
    /// List of maps in the assets directories
    pub maps: Vec<ConfigRecord>,
}
