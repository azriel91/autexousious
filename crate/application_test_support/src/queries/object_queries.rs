use amethyst::{
    assets::{AssetStorage, Handle, Prefab},
    ecs::World,
};
use asset_model::{config::AssetSlug, loaded::SlugAndHandle};
use character_loading::CharacterPrefab;
use character_model::loaded::CharacterObjectWrapper;
use object_loading::ObjectPrefab;

/// Functions to retrieve object data from a running world.
#[derive(Debug)]
pub struct ObjectQueries;

impl ObjectQueries {
    // TODO: Implement this for all game object types.
    /// Returns the `Handle<O::ObjectWrapper>` for the specified asset slug.
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
        world: &mut World,
        asset_slug: &AssetSlug,
    ) -> Handle<CharacterObjectWrapper> {
        let snh = SlugAndHandle::from((&*world, asset_slug.clone()));
        let game_object_prefab_assets =
            world.read_resource::<AssetStorage<Prefab<CharacterPrefab>>>();
        let game_object_prefab = game_object_prefab_assets
            .get(&snh.handle)
            .expect("Expected game object prefab to be loaded.");

        let object_prefab = &game_object_prefab
            .entities()
            .next()
            .expect("Expected game object main entity to exist.")
            .data()
            .expect("Expected game object prefab to contain data.")
            .object_prefab;

        if let ObjectPrefab::Handle(handle) = object_prefab {
            handle.clone()
        } else {
            panic!("Expected bat object prefab to be loaded.")
        }
    }
}
