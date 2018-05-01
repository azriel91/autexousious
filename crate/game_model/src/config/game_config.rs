use std::collections::HashMap;

use object_model::loaded;
use object_model::ObjectType;

/// Top level configuration structure for game configuration.
#[derive(Constructor, Debug, Default)]
pub struct GameConfig {
    /// Map of object type to the loaded objects defined for that type.
    pub loaded_objects_by_type: HashMap<ObjectType, Vec<loaded::Object>>,
}
