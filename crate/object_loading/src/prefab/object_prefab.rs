use std::{fmt::Debug, mem};

use amethyst::{
    assets::{AssetStorage, Handle, Loader, PrefabData, ProgressCounter},
    ecs::{Entity, Read, ReadExpect, WriteStorage},
    Error,
};
use object_model::{config::ObjectAssetData, loaded::GameObject};
use serde::{Deserialize, Serialize};

use crate::{
    ObjectAnimationStorages, ObjectComponentStorages, ObjectEntityAugmenter, ObjectPrefabError,
};

/// Sequence for volumes that can be hit.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ObjectPrefab<O>
where
    O: GameObject,
{
    /// Assets needed to load an object.
    ///
    /// This contains a handle to the game object data, as well as the sprite sheet handles.
    #[serde(skip)]
    Data(ObjectAssetData<O::Definition>),
    /// Already loaded handle.
    #[serde(skip)]
    Handle(Handle<O::ObjectWrapper>),
    /// Temporary value used during loading.
    #[serde(skip)]
    Invalid,
}

impl<'s, O> PrefabData<'s> for ObjectPrefab<O>
where
    O: GameObject,
    O::ObjectWrapper: Debug,
{
    type SystemData = (
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<O::ObjectWrapper>>,
        WriteStorage<'s, Handle<O::ObjectWrapper>>,
        ObjectComponentStorages<'s, O::SequenceId>,
        ObjectAnimationStorages<'s, O::SequenceId>,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (
            loader,
            object_wrapper_assets,
            object_wrapper_handles,
            object_component_storages,
            object_animation_storages,
        ): &mut Self::SystemData,
        _: &[Entity],
    ) -> Result<(), Error> {
        let object_wrapper_handle = match self {
            ObjectPrefab::Data(object_asset_data) => {
                loader.load_from_data(object_asset_data.clone(), (), &object_wrapper_assets)
            }
            ObjectPrefab::Handle(handle) => handle.clone(),
            ObjectPrefab::Invalid => {
                panic!("This variant should not be instantiated by consumers.")
            }
        };
        let object_wrapper = object_wrapper_assets
            .get(&object_wrapper_handle)
            .ok_or_else(|| ObjectPrefabError::NotLoaded {
                object_wrapper_handle: object_wrapper_handle.clone(),
            })?;

        object_wrapper_handles.insert(entity, object_wrapper_handle.clone())?;

        ObjectEntityAugmenter::augment(
            entity,
            object_component_storages,
            object_animation_storages,
            object_wrapper,
        );

        Ok(())
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        (loader, object_wrapper_assets, _, _, _): &mut Self::SystemData,
    ) -> Result<bool, Error> {
        let (self_, needs_loading_result) = match mem::replace(self, ObjectPrefab::Invalid) {
            ObjectPrefab::Data(object_asset_data) => {
                let handle =
                    loader.load_from_data(object_asset_data, progress, &object_wrapper_assets);
                (ObjectPrefab::Handle(handle), Ok(true))
            }
            value @ ObjectPrefab::Handle(..) => (value, Ok(false)),
            ObjectPrefab::Invalid => {
                panic!("This variant should not be instantiated by consumers.")
            }
        };
        *self = self_;

        needs_loading_result
    }
}
