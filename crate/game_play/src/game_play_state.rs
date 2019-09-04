use amethyst::{
    core::{math::Vector3, SystemBundle, Transform},
    ecs::{Builder, DispatcherBuilder, Entity, Read, World, WorldExt},
    input::{is_key_down, VirtualKeyCode},
    renderer::camera::{Camera, Projection},
    shred::Dispatcher,
    utils::{
        ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates},
        removal::{self, Removal},
    },
    window::ScreenDimensions,
    GameData, State, StateData, Trans,
};
use application_event::AppEvent;
use camera_model::play::CameraZoomDimensions;
use derivative::Derivative;
use derive_new::new;
use game_model::play::GameEntities;
use game_play_model::{GamePlayEntityId, GamePlayEvent, GamePlayStatus};
use log::{debug, info};
use state_registry::StateId;

use crate::GamePlayBundle;

/// Depth the camera can see.
const CAMERA_DEPTH: f32 = 2000.;

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
        let (window_width, window_height) = {
            let screen_dimensions = world.read_resource::<ScreenDimensions>();
            (screen_dimensions.width(), screen_dimensions.height())
        };
        let (zoom_width, zoom_height) = {
            world.setup::<Read<'_, CameraZoomDimensions>>();
            let camera_zoom_dimensions = world.read_resource::<CameraZoomDimensions>();
            (camera_zoom_dimensions.width, camera_zoom_dimensions.height)
        };

        // Camera translation from origin.
        //
        // The Z coordinate of the camera is how far along it should be before it faces the
        // entities. If an entity's Z coordinate is greater than the camera's Z coordinate, it will
        // be culled.
        //
        // By using `::std::f32::MAX` here, we ensure that all entities will be in the camera's
        // view.
        let translation = Vector3::new(zoom_width / 2., zoom_height / 2., CAMERA_DEPTH / 2.);
        let transform = Transform::from(translation);

        let world_coordinates = CameraOrthoWorldCoordinates {
            left: -zoom_width / 2.,
            right: zoom_width / 2.,
            bottom: -zoom_height / 2.,
            top: zoom_height / 2.,
        };
        let mut camera_ortho = CameraOrtho::normalized(CameraNormalizeMode::Contain);
        camera_ortho.world_coordinates = world_coordinates;

        let camera = world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                -window_width / 2.,
                window_width / 2.,
                -window_height / 2.,
                window_height / 2.,
                0.,
                // The distance that the camera can see. Since the camera is moved to the maximum Z
                // position, we also need to give it maximum Z viewing distance to ensure it can see
                // all entities in front of it.
                CAMERA_DEPTH,
            )))
            .with(camera_ortho)
            .with(transform)
            .build();

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
