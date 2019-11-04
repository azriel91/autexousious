use std::{collections::HashMap, path::Path};

use asset_model::config::{AssetRecord, AssetType};
use heck::SnakeCase;
use object_type::ObjectType;
use strum::IntoEnumIterator;

use crate::{AssetIndexingUtils, DirTraverse};

/// Indexes object types' assets.
#[derive(Debug)]
pub struct ObjectIndexer;

impl ObjectIndexer {
    /// Returns `AssetRecord`s for each of the objects in the namespace.
    ///
    /// # Parameters
    ///
    /// * `namespace`: Namespace that the objects reside in.
    /// * `object_types_dir`: Directory containing all object types' assets.
    pub fn index(namespace: &str, object_types_dir: &Path) -> HashMap<AssetType, Vec<AssetRecord>> {
        ObjectType::iter().fold(HashMap::new(), |mut objects_by_type, object_type| {
            let object_type_dir = object_types_dir.join(&object_type.to_string().to_snake_case());
            let object_dirs = DirTraverse::child_directories(&object_type_dir);

            objects_by_type.insert(
                AssetType::Object(object_type),
                object_dirs
                    .into_iter()
                    .filter_map(|object_dir| {
                        AssetIndexingUtils::asset_record(namespace.to_string(), object_dir)
                    })
                    .collect::<Vec<_>>(),
            );

            objects_by_type
        })
    }
}
