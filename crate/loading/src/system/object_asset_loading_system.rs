use std::{
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader, ProgressCounter},
    ecs::{Read, ReadExpect, System, World, Write},
    renderer::{SpriteSheet, Texture},
    shred::{ResourceId, SystemData},
};
use asset_loading::{AssetDiscovery, TomlFormat};
use asset_model::config::{AssetIndex, AssetRecord};
use derivative::Derivative;
use derive_new::new;
use game_model::loaded::GameObjectPrefabs;
use log::debug;
use object_loading::ObjectLoadingStatus;
use object_model::{config::ObjectAssetData, loaded::GameObject};
use object_prefab::GameObjectPrefab;
use serde::Deserialize;
use sprite_loading::SpriteLoader;
use sprite_model::config::SpritesDefinition;
use typename::TypeName as TypeNameTrait;
use typename_derive::TypeName;

use crate::ObjectAssetHandles;

/// Loads game object assets.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug(bound = ""))]
pub struct ObjectAssetLoadingSystem<O, Pf, St>
where
    O: GameObject + TypeNameTrait,
    O::Definition: Debug + for<'de> Deserialize<'de>,
    Pf: for<'p> GameObjectPrefab<'p, GameObject = O>
        + Debug
        + TypeNameTrait
        + Send
        + Sync
        + 'static,
    St: Debug + Default + Deref<Target = ObjectLoadingStatus> + DerefMut + Send + Sync + 'static,
{
    /// Path to the assets directory.
    assets_dir: PathBuf,
    /// Tracks loaded assets.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    progress_counter: ProgressCounter,

    // Fields below here are used during loading.
    /// Index of assets in the assets directory.
    #[new(default)]
    asset_index: Option<AssetIndex>,
    /// Assets that have not finished loading.
    #[new(default)]
    assets_in_progress: HashMap<AssetRecord, ObjectAssetHandles<O::Definition>>,
    /// Assets that been loaded, but the prefabs have not.
    #[new(default)]
    prefabs_in_progress: HashMap<AssetRecord, Handle<Prefab<Pf>>>,

    // Marker.
    phantom_data: PhantomData<St>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectAssetLoadingSystemData<'s, O, Pf, St>
where
    O: GameObject,
    Pf: for<'p> GameObjectPrefab<'p, GameObject = O>
        + Debug
        + TypeNameTrait
        + Send
        + Sync
        + 'static,
    St: Debug + Default + Deref<Target = ObjectLoadingStatus> + DerefMut + Send + Sync + 'static,
{
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    loader: ReadExpect<'s, Loader>,
    /// `AssetStorage` for `O::Definition`s.
    #[derivative(Debug = "ignore")]
    game_object_definition_assets: Read<'s, AssetStorage<O::Definition>>,
    /// `AssetStorage` for `SpritesDefinition`s.
    #[derivative(Debug = "ignore")]
    sprites_definition_assets: Read<'s, AssetStorage<SpritesDefinition>>,
    /// `AssetStorage` for `Texture`s.
    #[derivative(Debug = "ignore")]
    texture_assets: Read<'s, AssetStorage<Texture>>,
    /// `AssetStorage` for `SpriteSheet`s.
    #[derivative(Debug = "ignore")]
    sprite_sheet_assets: Read<'s, AssetStorage<SpriteSheet>>,
    /// `PrefabLoader` system data.
    #[derivative(Debug = "ignore")]
    game_object_prefab_loader: PrefabLoader<'s, Pf>,
    /// `AssetStorage` for `Prefab<Pf>`s.
    #[derivative(Debug = "ignore")]
    game_object_prefab_assets: Read<'s, AssetStorage<Prefab<Pf>>>,
    /// `GameObjectPrefabs<Pf>` resource.
    #[derivative(Debug = "ignore")]
    game_object_prefabs: Write<'s, GameObjectPrefabs<Pf>>,
    /// `St` resource.
    #[derivative(Debug = "ignore")]
    object_loading_status: Write<'s, St>,
}

impl<'s, O, Pf, St> System<'s> for ObjectAssetLoadingSystem<O, Pf, St>
where
    O: GameObject + TypeNameTrait,
    O::Definition: Debug + for<'de> Deserialize<'de>,
    Pf: for<'p> GameObjectPrefab<'p, GameObject = O>
        + Debug
        + TypeNameTrait
        + Send
        + Sync
        + 'static,
    St: Debug + Default + Deref<Target = ObjectLoadingStatus> + DerefMut + Send + Sync + 'static,
{
    type SystemData = ObjectAssetLoadingSystemData<'s, O, Pf, St>;

    fn run(
        &mut self,
        ObjectAssetLoadingSystemData {
            loader,
            game_object_definition_assets,
            sprites_definition_assets,
            texture_assets,
            sprite_sheet_assets,
            game_object_prefab_loader,
            game_object_prefab_assets,
            mut game_object_prefabs,
            mut object_loading_status,
        }: Self::SystemData,
    ) {
        // TODO: Do a diff between existing index and directory based on a file watch / notify.
        // TODO: See <https://github.com/polachok/derive-diff>
        if self.asset_index.is_none() {
            let asset_index = AssetDiscovery::asset_index(&self.assets_dir);
            debug!("Indexed assets: {:?}", &asset_index);

            // Borrow self piecewise.
            let assets_in_progress = &self.assets_in_progress;
            let prefabs_in_progress = &self.prefabs_in_progress;
            let progress_counter = &mut self.progress_counter;

            let asset_records = asset_index
                .objects
                .get(&O::OBJECT_TYPE)
                .cloned()
                .unwrap_or_else(Vec::new);

            let new_asset_records = asset_records.into_iter().filter(|asset_record| {
                !(assets_in_progress.contains_key(asset_record)
                    || prefabs_in_progress.contains_key(asset_record)
                    || game_object_prefabs.contains_key(&asset_record.asset_slug))
            });

            let new_object_asset_handles = new_asset_records
                .map(|asset_record| {
                    let object_asset_handles = Self::asset_record_to_handles(
                        progress_counter,
                        &loader,
                        &game_object_definition_assets,
                        &sprites_definition_assets,
                        &asset_record,
                    );
                    (asset_record, object_asset_handles)
                })
                // Need to collect the output, otherwise `self` is still borrowed when
                // we wish to extend `assets_in_progress`.
                .collect::<Vec<_>>();

            self.assets_in_progress.extend(new_object_asset_handles);
            self.asset_index = Some(asset_index);
        }

        // Check if any of `assets_in_progress` have completed loading, and move them to
        // `prefabs_in_progress`
        //
        // TODO: `HashMap` needs a `drain_filter` implementation. Check the following issue:
        // TODO: <https://github.com/rust-lang/rust/issues/43244>
        let new_map = HashMap::with_capacity(self.assets_in_progress.len());
        let mut assets_in_progress = mem::replace(&mut self.assets_in_progress, new_map);
        assets_in_progress
            .drain()
            .for_each(|(asset_record, object_asset_handles)| {
                if let (true, Some(sprites_definition)) = (
                    game_object_definition_assets
                        .get(&object_asset_handles.game_object_definition_handle)
                        .is_some(),
                    sprites_definition_assets.get(&object_asset_handles.sprites_definition_handle),
                ) {
                    let sprite_sheet_handles = SpriteLoader::load(
                        &mut self.progress_counter,
                        &loader,
                        &texture_assets,
                        &sprite_sheet_assets,
                        &sprites_definition,
                        &asset_record.path,
                    )
                    .expect("Failed to load textures and sprite sheets.");

                    let object_asset_data = ObjectAssetData::new(
                        object_asset_handles.game_object_definition_handle.clone(),
                        sprite_sheet_handles,
                    );
                    let game_object_prefab = Pf::new(object_asset_data);

                    let game_object_prefab_handle = game_object_prefab_loader.load_from_data(
                        Prefab::new_main(game_object_prefab),
                        &mut self.progress_counter,
                    );

                    self.prefabs_in_progress
                        .insert(asset_record, game_object_prefab_handle);
                } else {
                    self.assets_in_progress
                        .insert(asset_record, object_asset_handles);
                }
            });

        // Check if any of `prefabs_in_progress` have completed loading, and move them to
        // `game_object_prefabs`
        //
        // TODO: Split into separate System, since this borrows `GameObjectPrefabs` mutably.
        //
        // TODO: `HashMap` needs a `drain_filter` implementation. Check the following issue:
        // TODO: <https://github.com/rust-lang/rust/issues/43244>
        let new_map = HashMap::with_capacity(self.prefabs_in_progress.len());
        let mut prefabs_in_progress = mem::replace(&mut self.prefabs_in_progress, new_map);
        prefabs_in_progress
            .drain()
            .for_each(|(asset_record, prefab_handle)| {
                if game_object_prefab_assets.get(&prefab_handle).is_some() {
                    game_object_prefabs.insert(asset_record.asset_slug, prefab_handle);
                } else {
                    self.prefabs_in_progress.insert(asset_record, prefab_handle);
                }
            });

        // Unique asset handles are considered to be assets that failed to load:
        // <https://github.com/amethyst/amethyst/blob/v0.10.0/amethyst_assets/src/storage.rs#L190>
        //
        // `ProgressCounter#is_complete()` then never returns `true`:
        // <https://github.com/amethyst/amethyst/blob/v0.10.0/amethyst_assets/src/progress.rs#L89>
        //
        // As a compromise, we just check if there are no assets that are still loading.
        **object_loading_status = if self.progress_counter.num_loading() == 0 {
            ObjectLoadingStatus::Complete
        } else {
            debug!(
                "Loading progress: {}/{}",
                self.progress_counter.num_finished(),
                self.progress_counter.num_assets()
            );

            ObjectLoadingStatus::InProgress
        };
    }
}

impl<O, Pf, St> ObjectAssetLoadingSystem<O, Pf, St>
where
    O: GameObject + TypeNameTrait,
    O::Definition: Debug + for<'de> Deserialize<'de>,
    Pf: for<'p> GameObjectPrefab<'p, GameObject = O>
        + Debug
        + TypeNameTrait
        + Send
        + Sync
        + 'static,
    St: Debug + Default + Deref<Target = ObjectLoadingStatus> + DerefMut + Send + Sync + 'static,
{
    /// Initiates the asset loading for an object, and returns the handles.
    ///
    /// # Parameters
    ///
    /// * `progress_counter`: `ProgressCounter` to track loading progress.
    /// * `loader`: `Loader` to load assets.
    /// * `game_object_definition_assets`: `AssetStorage` of the `GameObjectDefinition`.
    /// * `sprites_definition_assets`: `AssetStorage` of the `SpritesDefinition`.
    /// * `asset_record`: The asset record of the object.
    fn asset_record_to_handles(
        progress_counter: &mut ProgressCounter,
        loader: &Loader,
        game_object_definition_assets: &AssetStorage<O::Definition>,
        sprites_definition_assets: &AssetStorage<SpritesDefinition>,
        asset_record: &AssetRecord,
    ) -> ObjectAssetHandles<O::Definition> {
        debug!(
            "Loading `{}` from: `{}`",
            asset_record.asset_slug,
            asset_record.path.display()
        );

        let game_object_definition_handle = loader.load(
            asset_record
                .path
                .join("object.toml")
                .to_str()
                .expect("Expected path to be valid unicode."),
            TomlFormat,
            &mut *progress_counter,
            game_object_definition_assets,
        );
        let sprites_definition_handle = loader.load(
            asset_record
                .path
                .join("sprites.toml")
                .to_str()
                .expect("Expected path to be valid unicode."),
            TomlFormat,
            &mut *progress_counter,
            sprites_definition_assets,
        );

        ObjectAssetHandles::new(game_object_definition_handle, sprites_definition_handle)
    }
}
