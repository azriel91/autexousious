use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

use amethyst::{
    assets::AssetStorage,
    ecs::prelude::*,
    input::is_key_down,
    prelude::*,
    renderer::{Event, VirtualKeyCode},
};
use character_selection::{CharacterEntityControl, CharacterSelection};
use game_model::play::GameEntities;
use map_model::loaded::Map;
use map_selection::MapSelection;
use object_model::{
    entity::{Kinematics, Position, Velocity},
    ObjectType,
};

use CharacterEntitySpawner;
use MapLayerEntitySpawner;

/// `State` where game play takes place.
#[derive(Derivative, Default, new)]
#[derivative(Debug)]
pub struct GameLoadingState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>> + 'static,
{
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: Box<F>,
    /// State specific dispatcher.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    dispatcher: Option<Dispatcher<'static, 'static>>,
    /// Data type used by this state and the returned state (see `StateData`).
    state_data: PhantomData<GameData<'a, 'b>>,
}

impl<'a, 'b, F, S> GameLoadingState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>> + 'static,
{
    /// Sets up the dispatcher for this state.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to operate on.
    fn initialize_dispatcher(&mut self, world: &mut World) {
        let dispatcher_builder = DispatcherBuilder::new();
        // dispatcher_builder.add(
        //     MapSelectionSystem::new(),
        //     &MapSelectionSystem::type_name(),
        //     &[],
        // );
        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut world.res);
        self.dispatcher = Some(dispatcher);
    }

    /// Initializes map layers and returns the entities along with the map's width and height.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` that contains the `MapSelection`.
    fn initialize_map(world: &mut World) -> (Vec<Entity>, f32, f32) {
        let map_handle = world
            .read_resource::<MapSelection>()
            .map_handle
            .as_ref()
            .expect("Expected map to be selected.")
            .clone();

        let map_layers = MapLayerEntitySpawner::spawn_world(world, &map_handle);

        // Used to determine where to spawn characters.
        let (width, height) = {
            let map_store = world.read_resource::<AssetStorage<Map>>();
            map_store
                .get(&map_handle)
                .map(|map| {
                    let bounds = &map.definition.header.bounds;
                    (bounds.width as f32, bounds.height as f32)
                }).expect("Expected map to be loaded.")
        };

        (map_layers, width, height)
    }

    /// Initializes and returns character entities.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` that contains the `CharacterSelection`.
    /// * `width`: Width of the map.
    /// * `height`: Height of the map.
    fn initialize_characters(world: &mut World, width: f32, height: f32) -> Vec<Entity> {
        // This `Position` moves the entity to the middle of a "screen wide" map.
        let position = Position::new(width / 2., height / 2., 0.);
        let kinematics = Kinematics::new(position, Velocity::default());

        // We need to collect this first because `world` needs to be borrowed immutably first, then
        // mutably later.
        let character_entities_to_spawn = {
            let character_selection = world.read_resource::<CharacterSelection>();
            character_selection
                .iter()
                .map(|(controller_id, character_index)| {
                    (
                        *character_index,
                        CharacterEntityControl::new(*controller_id),
                    )
                }).collect::<Vec<(usize, CharacterEntityControl)>>()
        };

        character_entities_to_spawn
            .into_iter()
            .map(|(character_index, character_entity_control)| {
                CharacterEntitySpawner::spawn_world(
                    world,
                    kinematics,
                    character_index,
                    character_entity_control,
                )
            }).collect::<Vec<Entity>>()
    }

    /// Initializes game object entities and map entities and adds them to the `World`.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` in which to initialize the game entities.
    fn initialize_entities(&mut self, world: &mut World) {
        let (map_layers, width, height) = Self::initialize_map(world);

        let mut objects = HashMap::new();
        objects.insert(
            ObjectType::Character,
            Self::initialize_characters(world, width, height),
        );

        let game_entities = GameEntities {
            objects,
            map_layers,
        };

        world.add_resource(game_entities);
    }

    /// Terminates the dispatcher.
    fn terminate_dispatcher(&mut self) {
        self.dispatcher = None;
    }
}

impl<'a, 'b, F, S> State<GameData<'a, 'b>> for GameLoadingState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>> + 'static,
{
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.initialize_dispatcher(&mut data.world);
        self.initialize_entities(&mut data.world);
    }

    fn on_stop(&mut self, _data: StateData<GameData<'a, 'b>>) {
        self.terminate_dispatcher();
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        if is_key_down(&event, VirtualKeyCode::Escape) {
            info!("Returning from `GameLoadingState`.");
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);

        // TODO: use systems to spawn entities instead of doing it on_start.
        // TODO: `Trans:Push` when we have a proper map selection menu.
        Trans::Switch((self.next_state_fn)())
    }
}
