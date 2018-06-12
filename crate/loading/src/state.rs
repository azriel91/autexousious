use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::PathBuf;

use amethyst::{self, prelude::*};
use application_ui::ThemeLoader;
use game_model::config::index_configuration;
use object_loading::CharacterLoader;
use object_model::{loaded::CharacterHandle, ObjectType};

/// `State` where resource loading takes place.
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
        Trans::None
    }

    fn fixed_update(&mut self, _data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        Trans::Switch(
            self.next_state
                .take()
                .expect("Expected `next_state` to be set"),
        )
    }
}
