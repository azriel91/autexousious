use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, ScreenDimensions, VirtualKeyCode, WindowEvent};
use amethyst::shred::Fetch;
use amethyst::ui::{Anchor, Anchored, UiText, UiTransform};
use application_ui::{FontVariant, Theme, ThemeLoader};

const FONT_SIZE: f32 = 25.;

pub struct TextState;

impl<'a, 'b> State<GameData<'a, 'b>> for TextState {
    fn on_start(&mut self, world: &mut World) {
        load_fonts(world);
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

fn load_fonts(world: &mut World) {
    ThemeLoader::load(world).expect("Failed to load fonts.");
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

        let screen_w = world.read_resource::<ScreenDimensions>().width();
        let text_w = screen_w / 3.;
        let text_h = 50.;

        font_tuples
            .into_iter()
            .map(|(font, text, y_offset)| {
                let text_transform = UiTransform::new(
                    text.to_string(),
                    text_w / 2. + 20.,
                    text_h / 2. + y_offset + 20.,
                    1.,
                    text_w,
                    text_h,
                    0,
                );

                let ui_text =
                    UiText::new(font.clone(), text.to_string(), [1., 1., 1., 1.], FONT_SIZE);
                (text_transform, ui_text)
            })
            .collect::<Vec<(UiTransform, UiText)>>()
    };

    for (text_transform, ui_text) in ui_text_components.into_iter() {
        world
            .create_entity()
            .with(text_transform)
            .with(ui_text)
            .with(Anchored::new(Anchor::TopLeft))
            .build();
    }
}

fn read_theme<'w>(world: &'w World) -> Fetch<'w, Theme> {
    world.read_resource::<Theme>()
}
