use amethyst::{
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{Event, ScreenDimensions, VirtualKeyCode},
    shred::Fetch,
    ui::{Anchor, UiText, UiTransform},
};
use application_ui::{FontVariant, Theme, ThemeLoader};

const FONT_SIZE: f32 = 25.;

pub struct TextState;

impl<'a, 'b> State<GameData<'a, 'b>> for TextState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        load_fonts(&mut data.world);
        initialize_text(&mut data.world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
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
                    Anchor::TopLeft,
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
            }).collect::<Vec<(UiTransform, UiText)>>()
    };

    for (text_transform, ui_text) in ui_text_components.into_iter() {
        world
            .create_entity()
            .with(text_transform)
            .with(ui_text)
            .build();
    }
}

fn read_theme<'w>(world: &'w World) -> Fetch<'w, Theme> {
    world.read_resource::<Theme>()
}
