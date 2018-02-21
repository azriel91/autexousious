use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, ScreenDimensions, VirtualKeyCode, WindowEvent};
use amethyst::ui::{FontHandle, UiResize, UiText, UiTransform};

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
    let (font_regular, font_bold, font_italic, font_bold_italic) = read_fonts(world);

    let mut fonts = vec![
        // font, text to display, y_offset
        (font_regular, "regular", 0.),
        (font_bold, "bold", 50.),
        (font_italic, "italic", 100.),
        (font_bold_italic, "bold_italic", 150.),
    ];

    fonts.drain(..).for_each(|(font, text, y_offset)| {
        let mut text_transform =
            UiTransform::new(text.to_string(), 20., y_offset + 20., 1., 400., 100., 0);
        let ui_text_size_fn = |_transform: &mut UiTransform, (_width, _height)| {};

        {
            let dim = world.read_resource::<ScreenDimensions>();
            ui_text_size_fn(&mut text_transform, (dim.width(), dim.height()));
        }

        world
            .create_entity()
            .with(text_transform)
            .with(UiText::new(
                font,
                text.to_string(),
                [1., 1., 1., 1.],
                FONT_SIZE,
            ))
            .with(UiResize(Box::new(ui_text_size_fn)))
            .build();
    });
}

type FH = FontHandle;
fn read_fonts(world: &mut World) -> (FH, FH, FH, FH) {
    use application_ui::FontVariant::{Bold, BoldItalic, Italic, Regular};
    (
        world.read_resource_with_id::<FH>(Regular as usize).clone(),
        world.read_resource_with_id::<FH>(Bold as usize).clone(),
        world.read_resource_with_id::<FH>(Italic as usize).clone(),
        world
            .read_resource_with_id::<FH>(BoldItalic as usize)
            .clone(),
    )
}
