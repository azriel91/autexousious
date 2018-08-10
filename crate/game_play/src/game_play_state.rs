use amethyst::{
    core::{
        cgmath::{Matrix4, Ortho, Vector3},
        transform::GlobalTransform,
    },
    ecs::prelude::*,
    input::is_key_down,
    prelude::*,
    renderer::{Camera, Event, Projection, ScreenDimensions, VirtualKeyCode},
};
use game_model::play::GameEntities;

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
    fn terminate_entities(&mut self, world: &mut World) {
        // This `allow` is needed because rustc evaluates that `game_entities` does not live long
        // enough when entities is constructed, so we need to bind entities to a variable.
        //
        // However, that triggers the clippy lint that we're binding and then returning. Pending:
        //
        // * <https://github.com/rust-lang-nursery/rust-clippy/issues/1524>
        // * <https://github.com/rust-lang-nursery/rust-clippy/issues/2904>
        #[allow(unknown_lints)]
        #[allow(let_and_return)]
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
    }

    /// Initializes a camera to view the game.
    fn initialize_camera(&mut self, world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        // Flip Z axis so that it is looking towards the screen, and positive Z values are visible.
        // If we don't do this, the camera is facing out of the screen, and so everything will be
        // behind the camera, and hence not drawn.
        let z_flip = Matrix4::from_nonuniform_scale(1., 1., -1.);
        // Camera is at origin.
        //
        // TODO: Using visibility sorting causes layers and objects to render strangely -- the depth
        // buffer uses the entity's z axis value to determine if the pixels should be drawn, whereas
        // the visibility sorting uses the distance from the camera, which is based on all X, Y, and
        // Z axes values. This means, if, in world coordinates, if an object entity moves further
        // away from the camera entity compared to a layer entity, the layer entity's sprite will
        // not be rendered behind the object entity's sprite.
        let translation = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
        let global_transform = GlobalTransform(translation * z_flip);

        let camera = world
            .create_entity()
            .with(Camera::from(Projection::Orthographic(Ortho {
                left: 0.0,
                right: width,
                top: height,
                bottom: 0.0,
                near: 0.0,
                far: 20000.,
            }))).with(global_transform)
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
            ).expect("Failed to delete camera entity.");
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for GamePlayState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.initialize_camera(&mut data.world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        if is_key_down(&event, VirtualKeyCode::Escape) {
            info!("Returning from `GamePlayState`.");
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.terminate_entities(&mut data.world);
        self.terminate_camera(&mut data.world);
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}
