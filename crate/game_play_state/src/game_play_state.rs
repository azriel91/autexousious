use amethyst::{
    core::{
        cgmath::{Matrix4, Vector3},
        transform::GlobalTransform,
    },
    ecs::prelude::*,
    input::is_key,
    prelude::*,
    renderer::{Camera, Event, Projection, ScreenDimensions, VirtualKeyCode},
};
use character_selection::{CharacterEntityControl, CharacterSelection};
use object_model::entity::{Kinematics, Position, Velocity};

use CharacterEntitySpawner;

/// `State` where game play takes place.
#[derive(Debug, Default)]
pub struct GamePlayState {
    /// Holds the entities in game.
    entities: Vec<Entity>,
    /// Camera entity
    camera: Option<Entity>,
}

impl GamePlayState {
    /// Returns a new `GamePlayState`.
    pub fn new() -> Self {
        Default::default()
    }

    fn initialize_entities(&mut self, world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

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
                })
                .collect::<Vec<(usize, CharacterEntityControl)>>()
        };

        character_entities_to_spawn.into_iter().for_each(
            |(character_index, character_entity_control)| {
                let entity = CharacterEntitySpawner::spawn_for_player(
                    world,
                    kinematics,
                    character_index,
                    character_entity_control,
                );

                self.entities.push(entity);
            },
        )
    }

    fn terminate_entities(&mut self, world: &mut World) {
        self.entities.drain(..).for_each(|entity| {
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

        let camera = world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0, width, height, 0.0,
            )))
            .with(GlobalTransform(Matrix4::from_translation(Vector3::new(
                0.0, 0.0, 1.0,
            ))))
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

impl<'a, 'b> State<GameData<'a, 'b>> for GamePlayState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.initialize_camera(&mut data.world);
        self.initialize_entities(&mut data.world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        if is_key(&event, VirtualKeyCode::Escape) {
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
