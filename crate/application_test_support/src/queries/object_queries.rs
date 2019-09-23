use amethyst::{
    assets::{AssetStorage, Handle, Prefab},
    ecs::{Entity, World, WorldExt},
};
use asset_model::{
    config::{AssetSlug, AssetType},
    loaded::{AssetId, SlugAndHandle},
};
use character_model::loaded::CharacterObjectWrapper;
use character_prefab::CharacterPrefab;
use game_model::play::GameEntities;
use object_prefab::{GameObjectPrefab, ObjectPrefab};
use object_type::ObjectType;

use crate::AssetQueries;

/// Functions to retrieve object data from a running world.
#[derive(Debug)]
pub struct ObjectQueries;

impl ObjectQueries {
    /// Returns the first entity in the World for the specified `ObjectType`.
    ///
    /// This is generally used with an application instantiated with `game_base`.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `object_type`: `ObjectType` for which to retrieve an entity.
    ///
    /// # Panics
    ///
    /// * Panics if the `GameEntities` resource does not exist.
    /// * Panics if there are no entities for the specified `ObjectType`.
    pub fn game_object_entity(world: &World, object_type: ObjectType) -> Entity {
        let game_entities = &*world.read_resource::<GameEntities>();
        let objects = game_entities.objects.get(&object_type);
        let object_entities = objects
            .unwrap_or_else(|| panic!("Expected entry for the `{}` object type.", object_type));
        *object_entities.iter().next().unwrap_or_else(|| {
            panic!(
                "No entities were initialized for the `{}` object type.\n\
                 Ensure you are using `game_base` or are using the `GameLoadingState` to set up \
                 the application.",
                object_type
            )
        })
    }

    /// Returns `Handle<Pf::GameObject>` for the specified asset slug.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Object slug whose `Handle<O>` to retrieve.
    pub fn game_object_handle<'s, Pf>(
        world: &World,
        asset_slug: &AssetSlug,
    ) -> Option<Handle<Pf::GameObject>>
    where
        Pf: GameObjectPrefab<'s> + Send + Sync + 'static,
    {
        let snh = SlugAndHandle::from((&*world, asset_slug.clone()));
        let game_object_prefab_assets = world.read_resource::<AssetStorage<Prefab<Pf>>>();
        let game_object_prefab = game_object_prefab_assets
            .get(&snh.handle)
            .expect("Expected game object prefab to be loaded.");

        let game_object_prefab = game_object_prefab
            .entities()
            .next()
            .expect("Expected game object main entity to exist.")
            .data()
            .expect("Expected game object prefab to contain data.");

        game_object_prefab.game_object_handle()
    }

    // TODO: Implement this for all game object types.
    /// Returns `Handle<O::ObjectWrapper>` for the specified asset slug.
    ///
    /// This function assumes the game object is instantiated in the world.
    ///
    /// There is no corresponding `object_wrapper` function because borrowing from the asset storage
    /// requires the `object_wrapper_assets` to be in the scope of the caller, for the lifetime of
    /// the asset storage reference to be valid.
    ///
    /// As a consolation, here is a code snippet you can copy:
    ///
    /// ```rust,ignore
    /// let object_wrapper_handle = Self::object_wrapper_handle(world, asset_slug);
    /// let object_wrapper_assets = world.read_resource::<AssetStorage<CharacterObjectWrapper>>();
    /// object_wrapper_assets
    ///     .get(&object_wrapper_handle)
    ///     .unwrap_or_else(|| panic!("Expected `{}` object wrapper to be loaded.", asset_slug))
    /// ```
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Object slug whose `Handle<O::ObjectWrapper>` to retrieve.
    pub fn object_wrapper_handle(
        world: &World,
        asset_slug: &AssetSlug,
    ) -> Handle<CharacterObjectWrapper> {
        let snh = SlugAndHandle::from((&*world, asset_slug.clone()));
        let game_object_prefab_assets =
            world.read_resource::<AssetStorage<Prefab<CharacterPrefab>>>();
        let game_object_prefab = game_object_prefab_assets
            .get(&snh.handle)
            .expect("Expected game object prefab to be loaded.");

        let object_prefab = game_object_prefab
            .entities()
            .next()
            .expect("Expected game object main entity to exist.")
            .data()
            .expect("Expected game object prefab to contain data.")
            .object_prefab();

        if let ObjectPrefab::Handle(handle) = object_prefab {
            handle.clone()
        } else {
            panic!("Expected bat object prefab to be loaded.")
        }
    }

    /// Returns the `AssetId` of the first character asset.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    pub fn first_character_asset_id(world: &World) -> AssetId {
        AssetQueries::first_id(world, &AssetType::Object(ObjectType::Character))
    }
}
