use std::sync::Arc;

use amethyst::{
    ecs::{Builder, Entity, World, WorldExt},
    shred::ParSeq,
    shrev::{EventChannel, ReaderId},
    ui::{Anchor, FontHandle, Interactable, UiText, UiTransform},
    GameData, State, StateData, StateEvent, Trans,
};
use application_menu::{MenuEvent, MenuItem};
use application_ui::{FontVariant, Theme, ThemeLoader};
use derivative::Derivative;
use log::debug;
use rayon;

use crate::main_menu::{self, UiEventHandlerSystem};

const FONT_SIZE: f32 = 25.;

/// Main menu with options to start a game or exit.
#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct MainMenuState {
    /// Dispatcher for UI handler system.
    #[derivative(Debug = "ignore")]
    dispatch: Option<ParSeq<Arc<rayon::ThreadPool>, UiEventHandlerSystem>>,
    /// ID of the reader for menu events.
    menu_event_reader: Option<ReaderId<MenuEvent<main_menu::Index>>>,
    /// Menu item entities, which we create / delete when the state is run / paused
    menu_items: Vec<Entity>,
}

impl MainMenuState {
    /// Returns a `MainMenuState`
    pub fn new() -> Self {
        Default::default()
    }

    fn load_theme(&self, world: &mut World) {
        ThemeLoader::load(world).expect("Failed to load fonts.");
    }

    fn initialize_menu_event_channel(&mut self, world: &mut World) {
        let mut menu_event_channel = EventChannel::<MenuEvent<main_menu::Index>>::with_capacity(20);
        let reader_id = menu_event_channel.register_reader();
        self.menu_event_reader.get_or_insert(reader_id);

        world.insert(menu_event_channel);
    }

    fn terminate_menu_event_channel(&mut self, _world: &mut World) {
        // By design there is no function to unregister a reader from an `EventChannel`.
        // Nor is there one to remove a resource from the `World`.

        self.menu_event_reader.take();
    }

    fn initialize_menu_items(&mut self, world: &mut World) {
        let font_bold = read_font(world);

        let item_indices = vec![main_menu::Index::StartGame, main_menu::Index::Exit];
        item_indices
            .into_iter()
            .enumerate()
            .for_each(|(order, index)| {
                let text_w = 400.;
                let text_h = 50.;
                let text_x = 20.;
                let text_y = -(order as f32 * text_h) - 20.;
                debug!("({}, {}, {}, {})", text_x, text_y, text_w, text_h);

                let text_transform = UiTransform::new(
                    index.title().to_string(),
                    Anchor::TopLeft,
                    Anchor::TopLeft,
                    text_x,
                    text_y,
                    1.,
                    text_w,
                    text_h,
                );

                let menu_item_entity = world
                    .create_entity()
                    .with(text_transform)
                    .with(UiText::new(
                        font_bold.clone(),
                        index.title().to_string(),
                        [1., 1., 1., 1.],
                        FONT_SIZE,
                    ))
                    .with(Interactable)
                    .with(MenuItem { index })
                    .build();

                self.menu_items.push(menu_item_entity);
            });
    }

    fn terminate_menu_items(&mut self, world: &mut World) {
        self.menu_items.drain(..).for_each(|menu_item| {
            world
                .delete_entity(menu_item)
                .expect("Failed to delete menu item.");
        });
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for MainMenuState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let mut dispatch = ParSeq::new(
            UiEventHandlerSystem::new(),
            (*data.world.read_resource::<Arc<rayon::ThreadPool>>()).clone(),
        );
        ParSeq::setup(&mut dispatch, &mut data.world);
        self.dispatch = Some(dispatch);

        self.load_theme(&mut data.world);

        self.initialize_menu_event_channel(&mut data.world);
        self.initialize_menu_items(&mut data.world);
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.terminate_menu_items(&mut data.world);
        self.terminate_menu_event_channel(&mut data.world);

        self.dispatch.take();
    }

    // Need to explicitly hide and show the menu items during pause and resume
    fn on_resume(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.initialize_menu_items(&mut data.world);
    }

    fn on_pause(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.terminate_menu_items(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world);
        self.dispatch.as_mut().unwrap().dispatch(&data.world);

        let menu_event_channel = &mut data
            .world
            .read_resource::<EventChannel<MenuEvent<main_menu::Index>>>();

        let mut reader_id = self
            .menu_event_reader
            .as_mut()
            .expect("Expected menu_event_reader to be set");
        let mut storage_iterator = menu_event_channel.read(&mut reader_id);
        match storage_iterator.next() {
            Some(event) => match *event {
                MenuEvent::Select(idx) => idx.trans(),
                MenuEvent::Close => Trans::Quit,
            },
            None => Trans::None,
        }
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
