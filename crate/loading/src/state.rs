use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

use amethyst;
use amethyst::prelude::*;
use game_model::config::GameConfig;
use game_model::config::index_configuration;
use object_model::ObjectType;
use object_model::loaded;

use object_loading::ObjectLoader;

/// `State` where resource loading takes place.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct State<T: amethyst::State + 'static> {
    /// Path to the assets directory.
    assets_dir: PathBuf,
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "T: Debug"))]
    next_state: Option<Box<T>>,
}

impl<'p, T: amethyst::State + 'static> State<T> {
    /// Returns a new `State`
    pub fn new(assets_dir: PathBuf, next_state: Box<T>) -> Self {
        State {
            assets_dir,
            next_state: Some(next_state),
        }
    }

    fn load_game_config(&mut self, world: &mut World) -> GameConfig {
        let configuration_index = index_configuration(&self.assets_dir);
        debug!("Indexed configuration: {:?}", &configuration_index);

        let mut object_loader = ObjectLoader::new();
        let loaded_objects_by_type = ObjectType::variants()
            .into_iter()
            .filter_map(|object_type| {
                configuration_index
                    .objects
                    .get(&object_type)
                    .map(|config_records| (object_type, config_records))
            })
            .map(|(object_type, config_records)| {
                // config_records is the list of records for one object type
                let loaded_objects = config_records
                    .iter()
                    .filter_map(|config_record| {
                        object_loader
                            .load_object(world, &object_type, config_record)
                            .ok()
                    })
                    .collect::<Vec<loaded::Object>>();

                (object_type, loaded_objects)
            })
            .collect::<HashMap<ObjectType, Vec<loaded::Object>>>();

        let game_config = GameConfig::new(loaded_objects_by_type);
        debug!("Game configuration: {:?}", &game_config);
        game_config
    }
}

impl<'p, T: amethyst::State + 'static> amethyst::State for State<T> {
    fn on_start(&mut self, world: &mut World) {
        // TODO: Start thread to load resources / assets.

        let game_config = self.load_game_config(world);

        world.add_resource(game_config);
    }

    fn fixed_update(&mut self, _world: &mut World) -> Trans {
        // TODO: Check state of resource / asset loading.
        // If it has loaded then `Trans::Switch`. Otherwise `Trans::None`

        Trans::Switch(
            self.next_state
                .take()
                .expect("Expected `next_state` to be set"),
        )
    }
}
