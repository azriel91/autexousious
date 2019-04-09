use amethyst::{
    assets::{Handle, PrefabData, ProgressCounter},
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
pub struct CharacterPrefab {
    /// Assets needed to load an object.
    ///
    /// This contains a handle to the game object data, as well as the sprite sheet handles.
    pub object_prefab: ObjectPrefab<Character>,
    /// Handle to the `CharacterDefinition`.
    pub character_definition_handle: Handle<CharacterDefinition>,
}

impl CharacterPrefab {
    /// Returns a new `CharacterPrefab`.
    pub fn new(object_asset_data: ObjectAssetData<CharacterDefinition>) -> Self {
        let character_definition_handle = object_asset_data.game_object_definition_handle.clone();
        let object_prefab = ObjectPrefab::Data(object_asset_data);

        CharacterPrefab {
            object_prefab,
            character_definition_handle,
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
        self.object_prefab
            .add_to_entity(entity, object_prefab_system_data, entities)?;

        CharacterEntityAugmenter::augment(entity, character_component_storages);

        Ok(())
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        (object_prefab_system_data, _character_component_storages): &mut Self::SystemData,
    ) -> Result<bool, Error> {
        self.object_prefab
            .load_sub_assets(progress, object_prefab_system_data)
    }
}

impl<'s> GameObjectPrefab<'s> for CharacterPrefab {
    type GameObject = Character;

    fn new(object_asset_data: ObjectAssetData<CharacterDefinition>) -> Self {
        CharacterPrefab::new(object_asset_data)
    }
}
