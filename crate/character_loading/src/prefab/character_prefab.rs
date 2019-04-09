use std::mem;

use amethyst::{
    assets::{PrefabData, ProgressCounter},
    ecs::Entity,
    Error,
};
use character_model::{config::CharacterDefinition, loaded::Character};
use object_loading::{GameObjectPrefab, ObjectPrefab};
use object_model::config::ObjectAssetData;
use typename_derive::TypeName;

use crate::{CharacterComponentStorages, CharacterEntityAugmenter};

/// Loads `CharacterDefinition`s and attaches components to character entities.
#[derive(Clone, Debug, PartialEq, TypeName)]
pub enum CharacterPrefab {
    /// Prefab has not been loaded from the assets.
    Data {
        /// The loaded `ObjectPrefab`.
        object_prefab: ObjectPrefab<Character>,
    },
    /// Prefab has been loaded from the assets.
    Loaded {
        /// The loaded `ObjectPrefab`.
        object_prefab: ObjectPrefab<Character>,
        /// The loaded `Character`.
        character: Character,
    },
    /// Temporary value used during loading.
    Invalid,
}

impl CharacterPrefab {
    /// Returns a new `CharacterPrefab`.
    ///
    /// # Parameters
    ///
    /// * `object_asset_data`: Assets needed to load an object.
    pub fn new(object_asset_data: ObjectAssetData<CharacterDefinition>) -> Self {
        CharacterPrefab::Data {
            object_prefab: ObjectPrefab::Data(object_asset_data),
        }
    }
}

impl<'s> PrefabData<'s> for CharacterPrefab {
    type SystemData = (
        <ObjectPrefab<Character> as PrefabData<'s>>::SystemData,
        CharacterComponentStorages<'s>,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (object_prefab_system_data, ref mut character_component_storages): &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<(), Error> {
        match self {
            CharacterPrefab::Loaded {
                object_prefab,
                character: _character,
            } => {
                object_prefab.add_to_entity(entity, object_prefab_system_data, entities)?;

                CharacterEntityAugmenter::augment(entity, character_component_storages);

                Ok(())
            }
            _ => panic!("Expected self to be in the `CharacterPrefab::Loaded` variant."),
        }
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        (object_prefab_system_data, _character_component_storages): &mut Self::SystemData,
    ) -> Result<bool, Error> {
        let (self_, needs_loading_result) = match mem::replace(self, CharacterPrefab::Invalid) {
            CharacterPrefab::Data { mut object_prefab } => {
                object_prefab.load_sub_assets(progress, object_prefab_system_data)?;

                if let ObjectPrefab::Handle(object_wrapper_handle) = &object_prefab {
                    let object_wrapper_handle = object_wrapper_handle.clone();
                    let character = Character::new(object_wrapper_handle);

                    (
                        CharacterPrefab::Loaded {
                            object_prefab,
                            character,
                        },
                        Ok(true),
                    )
                } else {
                    (
                        CharacterPrefab::Data { object_prefab },
                        Err(Error::from_string(String::from(
                            "Expected `object_prefab` to be `Handle` variant.",
                        ))),
                    )
                }
            }
            value @ CharacterPrefab::Loaded { .. } => (value, Ok(false)),
            CharacterPrefab::Invalid => unreachable!(),
        };
        *self = self_;

        needs_loading_result
    }
}

impl<'s> GameObjectPrefab<'s> for CharacterPrefab {
    type GameObject = Character;

    fn new(object_asset_data: ObjectAssetData<CharacterDefinition>) -> Self {
        CharacterPrefab::new(object_asset_data)
    }

    fn object_prefab(&self) -> &ObjectPrefab<Self::GameObject> {
        match self {
            CharacterPrefab::Data { object_prefab }
            | CharacterPrefab::Loaded { object_prefab, .. } => &object_prefab,
            _ => unreachable!(),
        }
    }
}
