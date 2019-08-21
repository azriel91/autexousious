use amethyst::{
    assets::{Loader, ProgressCounter},
    ecs::{World, WorldExt},
};
use asset_model::config::AssetRecord;
use assets_built_in::{MAP_BLANK, MAP_BLANK_SLUG};
use game_model::loaded::MapPrefabs;
use log::{debug, error};
use map_loading::MapLoader;
use map_model::loaded::MapHandle;

/// Provides functions to load game assets.
#[derive(Debug)]
pub struct AssetLoader;

impl AssetLoader {
    /// Loads map configuration into the `World` from the specified assets directory.
    ///
    /// When this function returns, the `World` will be populated with the `MapPrefabs` resource.
    ///
    /// The normal use case for `AssetLoader` is to use the `load` function which loads both objects
    /// and maps. This method is exposed for testing the loading itself.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the map assets into.
    /// * `progress_counter`: Tracker for loading progress.
    /// * `indexed_maps`: Index of map assets.
    pub fn load_maps(
        world: &mut World,
        progress_counter: &mut ProgressCounter,
        indexed_maps: Vec<AssetRecord>,
    ) {
        let mut map_prefabs = indexed_maps
            .into_iter()
            .filter_map(|asset_record| {
                let load_result = MapLoader::load(world, &asset_record.path);

                if let Err(e) = load_result {
                    error!("Failed to load map. Reason: \n\n```\n{}\n```", e);

                    None
                } else {
                    Some((asset_record.asset_slug, load_result.unwrap()))
                }
            })
            .collect::<MapPrefabs>();

        let map_handle: MapHandle = {
            let loader = world.read_resource::<Loader>();
            loader.load_from_data(MAP_BLANK.clone(), progress_counter, &world.read_resource())
        };

        map_prefabs.insert(MAP_BLANK_SLUG.clone(), map_handle);

        debug!("Loaded map assets: `{:?}`", map_prefabs);

        world.insert(map_prefabs);
    }
}
