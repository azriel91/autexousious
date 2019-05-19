use amethyst::{
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{ScreenDimensions, VirtualKeyCode},
    shred::Fetch,
    ui::{Anchor, UiText, UiTransform},
};
use application_ui::{FontVariant, Theme, ThemeLoader};

const FONT_SIZE: f32 = 25.;

pub struct TextState;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for TextState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        load_fonts(&mut data.world);
        initialize_text(&mut data.world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
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
            (fonts.get(&FontVariant::Regular).unwrap(), "regular"),
            (fonts.get(&FontVariant::Bold).unwrap(), "bold"),
            (fonts.get(&FontVariant::Italic).unwrap(), "italic"),
            (fonts.get(&FontVariant::BoldItalic).unwrap(), "bold_italic"),
        ];

        let screen_w = world.read_resource::<ScreenDimensions>().width();
        let text_w = screen_w / 3.;
        let text_h = 50.;

        font_tuples
            .into_iter()
            .enumerate()
            .map(|(n, (font, text))| {
                let text_transform = UiTransform::new(
                    text.to_string(),
                    Anchor::TopLeft,
                    Anchor::TopLeft,
                    text_w / 2. + 20.,
                    -(n as f32 * text_h) - 20.,
                    1.,
                    text_w,
                    text_h,
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
            .build();
    }
}

fn read_theme<'w>(world: &'w World) -> Fetch<'w, Theme> {
    world.read_resource::<Theme>()
}
