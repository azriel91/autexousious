use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

use amethyst;
use amethyst::prelude::*;
use application_ui::ThemeLoader;
use game_model::config::index_configuration;
use game_model::config::GameConfig;
use object_model::loaded;
use object_model::ObjectType;

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

    fn load_game_config(&mut self, world: &mut World) {
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
                        object_loader.load(world, &object_type, config_record).ok()
                    })
                    .collect::<Vec<loaded::Object>>();

                (object_type, loaded_objects)
            })
            .collect::<HashMap<ObjectType, Vec<loaded::Object>>>();

        let game_config = GameConfig::new(loaded_objects_by_type);
        debug!("Game configuration: {:?}", &game_config);

        world.add_resource(game_config);
    }
}

impl<'p, T: amethyst::State + 'static> amethyst::State for State<T> {
    fn on_start(&mut self, world: &mut World) {
        if let Err(e) = ThemeLoader::load(world) {
            let err_msg = format!("Failed to load theme: {}", e);
            error!("{}", &err_msg);
            panic!(err_msg);
        }
        self.load_game_config(world);
    }

    fn fixed_update(&mut self, _world: &mut World) -> Trans {
        Trans::Switch(
            self.next_state
                .take()
                .expect("Expected `next_state` to be set"),
        )
    }
}
