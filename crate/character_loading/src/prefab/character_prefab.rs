use amethyst::{
    assets::{PrefabData, ProgressCounter},
    ecs::Entity,
    Error,
};
use character_model::loaded::Character;
use derive_new::new;
use object_loading::{GameObjectPrefab, ObjectPrefab};
use serde::{Deserialize, Serialize};
use typename_derive::TypeName;

use crate::{CharacterComponentStorages, CharacterEntityAugmenter};

/// Loads `CharacterDefinition`s and attaches components to character entities.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypeName, new)]
pub struct CharacterPrefab {
    /// Assets needed to load an object.
    ///
    /// This contains a handle to the game object data, as well as the sprite sheet handles.
    pub object_prefab: ObjectPrefab<Character>,
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

    fn new(object_prefab: ObjectPrefab<Self::GameObject>) -> Self {
        CharacterPrefab::new(object_prefab)
    }
}
