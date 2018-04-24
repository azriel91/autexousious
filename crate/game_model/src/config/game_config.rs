use std::collections::HashMap;

use object_model::ObjectType;
use object_model::loaded;

/// Top level configuration structure for game configuration.
#[derive(Constructor, Debug)]
pub struct GameConfig {
    /// Map of object type to the loaded objects defined for that type.
    pub loaded_objects_by_type: HashMap<ObjectType, Vec<loaded::Object>>,
}
