use std::path::Path;

use amethyst::{
    assets::{Loader, Progress},
    prelude::*,
    renderer::ScreenDimensions,
};
use asset_loading::AssetDiscovery;
use game_model::config::AssetIndex;
use map_loading::MapLoader;
use map_model::{
    config::{MapBounds, MapDefinition, MapHeader},
    loaded::{Map, MapHandle, Margins},
};
use object_loading::CharacterLoader;
use object_model::{loaded::CharacterHandle, ObjectType};
use strum::IntoEnumIterator;

/// Provides functions to load game assets.
#[derive(Debug)]
pub struct AssetLoader;

impl AssetLoader {
    /// Loads game assets into the `World` from the specified assets directory.
    ///
    /// When this function returns, the `World` will be populated with the `Vec<CharacterHandle>`
    /// and `Vec<MapHandle>` resources.
    ///
    /// TODO: Use a top level game configuration object.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the game assets into.
    /// * `assets_dir`: Base directory containing all assets to load.
    /// * `progress`: Tracker for loading progress.
    pub fn load_game_config<P>(world: &mut World, assets_dir: &Path, progress: P)
    where
        P: Progress,
    {
        let asset_index = AssetDiscovery::asset_index(assets_dir);

        debug!("Indexed configuration: {:?}", &asset_index);

        Self::load_objects(world, &asset_index);
        Self::load_maps(world, progress, &asset_index);
    }

    /// Loads object configuration into the `World` from the specified assets directory.
    ///
    /// When this function returns, the `World` will be populated with the `Vec<CharacterHandle>`
    /// resource.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the game assets into.
    /// * `asset_index`: Index of all assets.
    pub fn load_objects(world: &mut World, asset_index: &AssetIndex) {
        ObjectType::iter()
            .filter_map(|object_type| {
                asset_index
                    .objects
                    .get(&object_type)
                    .map(|config_records| (object_type, config_records))
            }).for_each(|(object_type, config_records)| {
                // config_records is the list of records for one object type
                match object_type {
                    ObjectType::Character => {
                        let loaded_characters = config_records
                            .iter()
                            .filter_map(|config_record| {
                                debug!(
                                    "Loading character from: `{}`",
                                    config_record.directory.display()
                                );
                                let result = CharacterLoader::load(world, config_record);

                                if let Err(ref e) = result {
                                    error!("Failed to load character. Reason: \n\n```\n{}\n```", e);
                                }

                                result.ok()
                            }).collect::<Vec<CharacterHandle>>();

                        debug!("Loaded character handles: `{:?}`", loaded_characters);

                        world.add_resource(loaded_characters);
                    }
                };
            });
    }

    /// Loads object configuration into the `World` from the specified assets directory.
    ///
    /// When this function returns, the `World` will be populated with the `Vec<CharacterHandle>`
    /// resource.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the game assets into.
    /// * `progress`: Tracker for loading progress.
    /// * `asset_index`: Index of all assets.
    pub fn load_maps<P>(world: &mut World, progress: P, asset_index: &AssetIndex)
    where
        P: Progress,
    {
        let mut loaded_maps = asset_index
            .maps
            .iter()
            .filter_map(|config_record| MapLoader::load(world, &config_record.directory).ok())
            .collect::<Vec<MapHandle>>();

        // Default Blank Map
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let depth = 200;
        let bounds = MapBounds::new(0, 0, 0, width as u32, height as u32 - depth, depth);
        let header = MapHeader::new("Blank Screen".to_string(), bounds);
        let layers = Vec::new();
        let definition = MapDefinition::new(header, layers);
        let margins = Margins::from(definition.header.bounds);
        let map = Map::new(definition, margins, None, None);

        let map_handle: MapHandle = {
            let loader = world.read_resource::<Loader>();
            loader.load_from_data(map, progress, &world.read_resource())
        };

        loaded_maps.push(map_handle);

        debug!("Loaded map handles: `{:?}`", loaded_maps);

        world.add_resource(loaded_maps);
    }
}
