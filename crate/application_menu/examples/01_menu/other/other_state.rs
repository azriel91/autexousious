use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, ScreenDimensions, VirtualKeyCode, WindowEvent};
use amethyst::ui::{Anchor, Anchored, FontHandle, UiText, UiTransform};
use application_ui::{FontVariant, Theme};

const FONT_SIZE: f32 = 17.;

#[derive(Debug, Default)]
pub struct OtherState {
    /// Holds the info label.
    entity: Option<Entity>,
}

impl OtherState {
    pub fn new() -> Self {
        Default::default()
    }

    fn initialize_informative(&mut self, world: &mut World) {
        let font_bold = read_font(world);

        let screen_w = world.read_resource::<ScreenDimensions>().width();
        let text_w = screen_w;
        let text_h = 100.;

        let text_transform = UiTransform::new(
            "info".to_string(),
            text_w / 2. + 20.,
            text_h / 2. + 20.,
            1.,
            text_w,
            text_h,
            0,
        );

        let info_entity = world
            .create_entity()
            .with(text_transform)
            .with(UiText::new(
                font_bold.clone(),
                "Press [Escape] to return to the previous menu.".to_string(),
                [1., 1., 1., 1.],
                FONT_SIZE,
            ))
            .with(Anchored::new(Anchor::TopLeft))
            .build();

        self.entity.get_or_insert(info_entity);
    }

    fn terminate_informative(&mut self, world: &mut World) {
        world
            .delete_entity(self.entity.take().expect("Expected info_entity to be set."))
            .expect("Failed to delete info_entity.");
    }
}

impl State for OtherState {
    fn on_start(&mut self, world: &mut World) {
        self.initialize_informative(world);
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
                    info!("Returning from `OtherState`.");
                    Trans::Pop
                }
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, world: &mut World) {
        self.terminate_informative(world);
    }
}

fn read_font(world: &mut World) -> FontHandle {
    let theme = world.read_resource::<Theme>();
    theme
        .fonts
        .get(&FontVariant::Bold)
        .expect("Failed to get Bold font handle")
        .clone()
}
