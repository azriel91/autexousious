use std::mem;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, PrefabData, ProgressCounter},
    ecs::{Entity, Read, ReadExpect, WriteStorage},
    Error,
};
use character_model::{
    config::CharacterDefinition,
    loaded::{Character, CharacterControlTransitionsSequence, CharacterHandle},
};
use object_loading::{GameObjectPrefab, ObjectPrefab};
use object_model::config::ObjectAssetData;
use typename_derive::TypeName;

use crate::{
    CharacterComponentStorages, CharacterEntityAugmenter, CharacterLoader, CharacterLoaderParams,
};

/// Loads `CharacterDefinition`s and attaches components to character entities.
#[derive(Clone, Debug, PartialEq, TypeName)]
pub enum CharacterPrefab {
    /// Prefab has not been loaded from the assets.
    Data {
        /// The `ObjectPrefab`.
        object_prefab: ObjectPrefab<Character>,
        /// Handle to the `CharacterDefinition`.
        character_definition_handle: Handle<CharacterDefinition>,
    },
    /// Prefab has been loaded from the assets.
    Loaded {
        /// The loaded `ObjectPrefab`.
        object_prefab: ObjectPrefab<Character>,
        /// The loaded `Character`.
        character_handle: CharacterHandle,
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
        let character_definition_handle = object_asset_data.game_object_definition_handle.clone();

        CharacterPrefab::Data {
            object_prefab: ObjectPrefab::Data(object_asset_data),
            character_definition_handle,
        }
    }
}

impl<'s> PrefabData<'s> for CharacterPrefab {
    type SystemData = (
        <ObjectPrefab<Character> as PrefabData<'s>>::SystemData,
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<CharacterDefinition>>,
        Read<'s, AssetStorage<CharacterControlTransitionsSequence>>,
        Read<'s, AssetStorage<Character>>,
        WriteStorage<'s, CharacterHandle>,
        CharacterComponentStorages<'s>,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (
            object_prefab_system_data,
            _loader,
            _character_definition_assets,
            _control_transitions_sequence_assets,
            _character_assets,
            ref mut character_handles,
            ref mut character_component_storages,
        ): &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<(), Error> {
        match self {
            CharacterPrefab::Loaded {
                object_prefab,
                character_handle,
            } => {
                object_prefab.add_to_entity(entity, object_prefab_system_data, entities)?;

                character_handles
                    .insert(entity, character_handle.clone())
                    .expect("Failed to insert `CharacterHandle` component.");

                CharacterEntityAugmenter::augment(entity, character_component_storages);

                Ok(())
            }
            _ => panic!("Expected self to be in the `CharacterPrefab::Loaded` variant."),
        }
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        (
            object_prefab_system_data,
            loader,
            character_definition_assets,
            character_control_transitions_sequence_assets,
            character_assets,
            _character_handles,
            _character_component_storages,
        ): &mut Self::SystemData,
    ) -> Result<bool, Error> {
        let (self_, needs_loading_result) = match mem::replace(self, CharacterPrefab::Invalid) {
            CharacterPrefab::Data {
                mut object_prefab,
                character_definition_handle,
            } => {
                object_prefab.load_sub_assets(progress, object_prefab_system_data)?;

                if let ObjectPrefab::Handle(object_wrapper_handle) = &object_prefab {
                    let object_wrapper_handle = object_wrapper_handle.clone();

                    let character_loader_params = CharacterLoaderParams {
                        loader: &loader,
                        character_control_transitions_sequence_assets:
                            &character_control_transitions_sequence_assets,
                    };
                    let character_definition = character_definition_assets
                        .get(&character_definition_handle)
                        .expect("Expected `CharacterDefinition` to be loaded.");
                    let character = CharacterLoader::load(
                        character_loader_params,
                        character_definition,
                        object_wrapper_handle,
                    )?;
                    let character_handle =
                        loader.load_from_data(character, progress, character_assets);

                    (
                        CharacterPrefab::Loaded {
                            object_prefab,
                            character_handle,
                        },
                        Ok(true),
                    )
                } else {
                    (
                        CharacterPrefab::Data {
                            object_prefab,
                            character_definition_handle,
                        },
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

    fn game_object_handle(&self) -> Option<CharacterHandle> {
        if let CharacterPrefab::Loaded {
            character_handle, ..
        } = self
        {
            Some(character_handle.clone())
        } else {
            None
        }
    }

    fn object_prefab(&self) -> &ObjectPrefab<Self::GameObject> {
        match self {
            CharacterPrefab::Data { object_prefab, .. }
            | CharacterPrefab::Loaded { object_prefab, .. } => &object_prefab,
            _ => unreachable!(),
        }
    }
}
