use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, ScreenDimensions, VirtualKeyCode, WindowEvent};
use amethyst::shred::Fetch;
use amethyst::ui::{UiResize, UiText, UiTransform};
use application_ui::{FontVariant, Theme};

const FONT_SIZE: f32 = 25.;

pub struct TextState;

impl State for TextState {
    fn on_start(&mut self, world: &mut World) {
        initialize_text(world);
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
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }
}

fn initialize_text(world: &mut World) {
    let ui_text_components = {
        let theme = read_theme(world);

        let fonts = &theme.fonts;
        let font_tuples = vec![
            // font, text to display, y_offset
            (fonts.get(&FontVariant::Regular).unwrap(), "regular", 0.),
            (fonts.get(&FontVariant::Bold).unwrap(), "bold", 50.),
            (fonts.get(&FontVariant::Italic).unwrap(), "italic", 100.),
            (
                fonts.get(&FontVariant::BoldItalic).unwrap(),
                "bold_italic",
                150.,
            ),
        ];

        let (screen_w, screen_h) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        font_tuples
            .into_iter()
            .map(|(font, text, y_offset)| {
                let mut text_transform =
                    UiTransform::new(text.to_string(), 20., y_offset + 20., 1., 400., 100., 0);
                let ui_text_size_fn = |_transform: &mut UiTransform, (_width, _height)| {};
                ui_text_size_fn(&mut text_transform, (screen_w, screen_h));

                let ui_resize = UiResize(Box::new(ui_text_size_fn));
                let ui_text =
                    UiText::new(font.clone(), text.to_string(), [1., 1., 1., 1.], FONT_SIZE);
                (text_transform, ui_text, ui_resize)
            })
            .collect::<Vec<(UiTransform, UiText, UiResize)>>()
    };

    for (text_transform, ui_text, ui_resize) in ui_text_components.into_iter() {
        world
            .create_entity()
            .with(text_transform)
            .with(ui_text)
            .with(ui_resize)
            .build();
    }
}

fn read_theme<'w>(world: &'w World) -> Fetch<'w, Theme> {
    world.read_resource::<Theme>()
}
