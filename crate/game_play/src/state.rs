use amethyst;
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Event, KeyboardInput, Material, Projection, ScreenDimensions,
                         VirtualKeyCode, WindowEvent};
use amethyst_animation::{get_animation_set, AnimationCommand, EndControl};
use game_model::config::GameConfig;

/// `State` where game play takes place.
#[derive(Debug, Default)]
pub struct State {
    /// Holds the entities in game.
    entities: Vec<Entity>,
    /// Camera entity
    camera: Option<Entity>,
}

impl State {
    /// Returns a new `game_play::State`.
    pub fn new() -> Self {
        Default::default()
    }

    fn initialize_entities(&mut self, world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        // This `Transform` moves the sprites to the middle of the window
        let mut common_transform = Transform::default();
        common_transform.translation = Vector3::new(width / 2., height / 2., 0.);

        // TODO: Instead of cloning, build up a `Vec<Component>`.
        let loaded_objects_by_type = {
            let game_config = world.read_resource::<GameConfig>();
            &game_config.loaded_objects_by_type.clone()
        };
        loaded_objects_by_type.values().for_each(|objects| {
            objects.iter().for_each(|object| {
                let animation_handle = object.animations.first().unwrap().clone();
                let entity = world
                    .create_entity()
                    // The default `Material`, whose textures will be swapped based on the animation.
                    .with(object.default_material.clone())
                    // Shift sprite to some part of the window
                    .with(common_transform.clone())
                    // This defines the coordinates in the world, where the sprites should be drawn
                    // relative to the entity
                    .with(object.mesh.clone())
                    // Used by the engine to compute and store the rendered position.
                    .with(GlobalTransform::default())
                    .build();

                // We also need to trigger the animation, not just attach it to the entity
                let mut animation_control_set_storage = world.write_storage();
                let animation_set =
                    get_animation_set::<u32, Material>(&mut animation_control_set_storage, entity);
                let animation_id = 0;
                animation_set.add_animation(
                    animation_id,
                    &animation_handle,
                    EndControl::Loop(None),
                    30., // Rate at which the animation plays
                    AnimationCommand::Start,
                );

                self.entities.push(entity);
            });
        });
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

impl amethyst::State for State {
    fn on_start(&mut self, world: &mut World) {
        self.initialize_camera(world);
        self.initialize_entities(world);
    }

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    info!("Returning from `game_play::State`.");
                    Trans::Pop
                }
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, world: &mut World) {
        self.terminate_entities(world);
        self.terminate_camera(world);
    }
}
