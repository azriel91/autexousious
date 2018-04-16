use amethyst;
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::transform::GlobalTransform;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Event, KeyboardInput, Projection, ScreenDimensions,
                         VirtualKeyCode, WindowEvent};
use amethyst::ui::{Anchor, Anchored, FontHandle, UiText, UiTransform};

const FONT_SIZE: f32 = 17.;

/// `State` where game play takes place.
///
/// Current implementation is a place holder until this is properly developed.
#[derive(Debug, Default)]
pub struct State {
    /// Holds the info label.
    entity: Option<Entity>,
    /// Camera entity
    camera: Option<Entity>,
}

impl State {
    /// Returns a new `game_play::State`.
    pub fn new() -> Self {
        Default::default()
    }

    fn initialize_informative(&mut self, world: &mut World) {
        let font = read_font(world);

        let screen_w = {
            let dim = world.read_resource::<ScreenDimensions>();
            dim.width()
        };
        let text_w = screen_w / 2.;
        let text_h = 50.;
        let text_transform = UiTransform::new("info".to_string(), 20., 20., 1., text_w, text_h, 0);

        let info_entity = world
            .create_entity()
            .with(text_transform)
            .with(UiText::new(
                font,
                "Press [Escape] to return to the previous menu.".to_string(),
                [1., 1., 1., 1.],
                FONT_SIZE,
            ))
            .with(Anchored::new(Anchor::Middle))
            .build();

        self.entity.get_or_insert(info_entity);
    }

    fn terminate_informative(&mut self, world: &mut World) {
        world
            .delete_entity(self.entity.take().expect("Expected entity to be set."))
            .expect("Failed to delete entity.");
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
                0.0,
                width,
                height,
                0.0,
            )))
            .with(GlobalTransform(Matrix4::from_translation(
                Vector3::new(0.0, 0.0, 1.0).into(),
            )))
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
        self.initialize_informative(world);
        self.initialize_camera(world);
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
        self.terminate_camera(world);
        self.terminate_informative(world);
    }
}

fn read_font(world: &mut World) -> FontHandle {
    use application_ui::FontVariant::Regular;
    world
        .read_resource_with_id::<FontHandle>(Regular.into())
        .clone()
}
