use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::PathBuf;

use amethyst::{
    self,
    assets::{AssetStorage, Loader, ProgressCounter},
    prelude::*,
    renderer::ScreenDimensions,
};
use application_ui::ThemeLoader;
use game_model::config::{index_configuration, ConfigIndex};
use map_loading::MapLoader;
use map_model::{
    config::{MapBounds, MapDefinition, MapHeader},
    loaded::{Map, MapHandle, Margins},
};
use object_loading::CharacterLoader;
use object_model::{loaded::CharacterHandle, ObjectType};

/// `State` where resource loading takes place.
///
/// If you use this `State`, you **MUST** ensure that both the `ObjectLoadingBundle` and
/// `MapLoadingBundle`s are included in the application dispatcher that this `State` delegates to
/// to load the assets.
///
/// # Type Parameters
///
/// * `S`: State to return after loading is complete.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct State<'a, 'b, S>
where
    S: amethyst::State<GameData<'a, 'b>> + 'static,
{
    /// Path to the assets directory.
    assets_dir: PathBuf,
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "S: Debug"))]
    next_state: Option<Box<S>>,
    /// Tracks loaded assets.
    #[derivative(Debug = "ignore")]
    progress_counter: ProgressCounter,
    /// Lifetime tracker.
    state_data: PhantomData<amethyst::State<GameData<'a, 'b>>>,
}

impl<'a, 'b, S> State<'a, 'b, S>
where
    S: amethyst::State<GameData<'a, 'b>> + 'static,
{
    /// Returns a new `State`
    pub fn new(assets_dir: PathBuf, next_state: Box<S>) -> Self {
        State {
            assets_dir,
            next_state: Some(next_state),
            progress_counter: ProgressCounter::new(),
            state_data: PhantomData,
        }
    }

    fn load_game_config(&mut self, world: &mut World) {
        let configuration_index = index_configuration(&self.assets_dir);
        debug!("Indexed configuration: {:?}", &configuration_index);

        ObjectType::variants()
            .into_iter()
            .filter_map(|object_type| {
                configuration_index
                    .objects
                    .get(&object_type)
                    .map(|config_records| (object_type, config_records))
            })
            .for_each(|(object_type, config_records)| {
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
                                CharacterLoader::load(world, config_record).ok()
                            })
                            .collect::<Vec<CharacterHandle>>();

                        debug!("Loaded character handles: `{:?}`", loaded_characters);

                        world.add_resource(loaded_characters);
                    }
                };
            });

        self.load_maps(world, &configuration_index);
    }

    fn load_maps(&mut self, world: &mut World, configuration_index: &ConfigIndex) {
        let mut loaded_maps = configuration_index
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
            loader.load_from_data(map, &mut self.progress_counter, &world.read_resource())
        };

        loaded_maps.push(map_handle);

        debug!("Loaded map handles: `{:?}`", loaded_maps);

        world.add_resource(loaded_maps);
    }
}

impl<'a, 'b, S> amethyst::State<GameData<'a, 'b>> for State<'a, 'b, S>
where
    S: amethyst::State<GameData<'a, 'b>> + 'static,
{
    fn on_start(&mut self, mut data: StateData<GameData>) {
        if let Err(e) = ThemeLoader::load(&mut data.world) {
            let err_msg = format!("Failed to load theme: {}", e);
            error!("{}", &err_msg);
            panic!(err_msg);
        }
        self.load_game_config(&mut data.world);
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);

        if self.progress_counter.is_complete() {
            Trans::Switch(
                self.next_state
                    .take()
                    .expect("Expected `next_state` to be set"),
            )
        } else {
            warn!(
                "If loading never completes, please ensure that you have registered both the \
                 `ObjectLoadingBundle` and `MapLoadingBundle`s to the application dispatcher, as \
                 those provide the necessary `System`s to process the loaded assets."
            );
            debug!(
                "Loading progress: {}/{}",
                self.progress_counter.num_finished(),
                self.progress_counter.num_assets()
            );
            let map_store = data.world.read_resource::<AssetStorage<Map>>();
            let map_handles = data.world.read_resource::<Vec<MapHandle>>();
            map_handles.iter().for_each(|handle| {
                map_store.get(handle);
            });

            Trans::None
        }
    }
}
