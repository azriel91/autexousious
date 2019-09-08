use amethyst::{
    core::SystemBundle,
    ecs::{DispatcherBuilder, Entity, World, WorldExt},
    input::{is_key_down, VirtualKeyCode},
    shred::Dispatcher,
    utils::removal::{self, Removal},
    GameData, State, StateData, Trans,
};
use application_event::AppEvent;
use camera_play::CameraCreator;
use derivative::Derivative;
use derive_new::new;
use game_model::play::GameEntities;
use game_play_model::{GamePlayEntityId, GamePlayEvent, GamePlayStatus};
use log::{debug, info};
use state_registry::StateId;

use crate::GamePlayBundle;

/// `State` where game play takes place.
#[derive(Derivative, Default, new)]
#[derivative(Debug)]
pub struct GamePlayState {
    /// State specific dispatcher.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    dispatcher: Option<Dispatcher<'static, 'static>>,
    /// Camera entity
    #[new(default)]
    camera: Option<Entity>,
}

impl GamePlayState {
    /// Sets up the dispatcher for this state.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to operate on.
    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();

        GamePlayBundle::new()
            .build(world, &mut dispatcher_builder)
            .expect("Failed to register `GamePlayBundle`.");

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
    }

    /// Terminates the dispatcher.
    fn terminate_dispatcher(&mut self) {
        self.dispatcher = None;
    }

    fn terminate_entities(&mut self, world: &mut World) {
        // This `allow` is needed because rustc evaluates that `game_entities` does not live long
        // enough when entities is constructed, so we need to bind entities to a variable.
        //
        // However, that triggers the clippy lint that we're binding and then returning. Pending:
        //
        // * <https://github.com/rust-lang-nursery/rust-clippy/issues/1524>
        // * <https://github.com/rust-lang-nursery/rust-clippy/issues/2904>
        #[allow(clippy::let_and_return)]
        let entities = {
            let mut game_entities = world.write_resource::<GameEntities>();
            let entities = game_entities.drain().collect::<Vec<Entity>>();

            entities
        };
        entities.into_iter().for_each(|entity| {
            world
                .delete_entity(entity)
                .expect("Failed to delete game entity.");
        });

        removal::exec_removal(
            &*world.entities(),
            &world.read_storage::<Removal<GamePlayEntityId>>(),
            GamePlayEntityId::default(),
        );
    }

    /// Initializes a camera to view the game.
    fn initialize_camera(&mut self, world: &mut World) {
        let camera = CameraCreator::create_in_world(world);
        self.camera = Some(camera);
    }

    /// Terminates the camera.
    fn terminate_camera(&mut self, world: &mut World) {
        world
            .delete_entity(
                self.camera
                    .take()
                    .expect("Expected camera entity to be set."),
            )
            .expect("Failed to delete camera entity.");
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, AppEvent> for GamePlayState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world.insert(StateId::GamePlay);

        self.initialize_dispatcher(&mut data.world);
        self.initialize_camera(&mut data.world);

        data.world.insert(GamePlayStatus::Playing);
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.terminate_entities(&mut data.world);
        self.terminate_camera(&mut data.world);
        self.terminate_dispatcher();
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        data.world.insert(StateId::GamePlay);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: AppEvent,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        match event {
            AppEvent::Window(window_event) => {
                if is_key_down(&window_event, VirtualKeyCode::Escape) {
                    debug!("Returning from `GamePlayState`.");
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            AppEvent::GamePlay(game_play_event) => {
                match game_play_event {
                    GamePlayEvent::Return => {
                        debug!("Returning from `GamePlayState`.");
                        Trans::Pop
                    }
                    GamePlayEvent::Restart => {
                        // TODO: Switch to `GameLoadingState`
                        Trans::None
                    }
                    GamePlayEvent::Pause => {
                        data.world.insert(GamePlayStatus::Paused);
                        Trans::None
                    }
                    GamePlayEvent::Resume => {
                        data.world.insert(GamePlayStatus::Playing);
                        Trans::None
                    }
                    GamePlayEvent::End => {
                        info!("Game play ended!");
                        data.world.insert(GamePlayStatus::Ended);
                        Trans::None
                    }
                    GamePlayEvent::EndStats => {
                        // TODO: `GamePlayStats` state.
                        Trans::Pop
                    }
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        // Note: The built-in dispatcher must be run before the state specific dispatcher as the
        // `"input_system"` is registered in the main dispatcher, and is a dependency of the
        // `ControllerInputUpdateSystem`.
        data.data.update(&data.world);
        self.dispatcher.as_mut().unwrap().dispatch(&data.world);
        Trans::None
    }
}
