use amethyst::{
    ecs::prelude::*,
    input::is_key_down,
    prelude::*,
    renderer::{ScreenDimensions, VirtualKeyCode},
    ui::{Anchor, FontHandle, UiText, UiTransform},
};
use application_ui::{FontVariant, Theme};
use log::info;

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
            Anchor::TopLeft,
            Anchor::TopLeft,
            20.,
            -text_h - 20.,
            1.,
            text_w,
            text_h,
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
            .build();

        self.entity.get_or_insert(info_entity);
    }

    fn terminate_informative(&mut self, world: &mut World) {
        world
            .delete_entity(self.entity.take().expect("Expected info_entity to be set."))
            .expect("Failed to delete info_entity.");
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for OtherState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.initialize_informative(&mut data.world);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                info!("Returning from `OtherState`.");
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.terminate_informative(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world);
        Trans::None
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
