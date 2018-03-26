use std::sync::Arc;

use amethyst;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::ScreenDimensions;
use amethyst::shred::ParSeq;
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{FontHandle, MouseReactive, UiResize, UiText, UiTransform};
use application_menu::{MenuEvent, MenuItem};
use rayon;

use main_menu::{self, UiEventHandlerSystem};

const FONT_SIZE: f32 = 25.;

/// Main menu with options to start a game or exit.
#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct State {
    /// Dispatcher for UI handler system.
    #[derivative(Debug = "ignore")]
    dispatch: Option<ParSeq<Arc<rayon::ThreadPool>, UiEventHandlerSystem>>,
    /// ID of the reader for menu events.
    menu_event_reader: Option<ReaderId<MenuEvent<main_menu::Index>>>,
    /// Menu item entities, which we create / delete when the state is run / paused
    menu_items: Vec<Entity>,
}

impl State {
    /// Returns a `State`
    pub fn new() -> Self {
        Default::default()
    }

    fn initialize_menu_event_channel(&mut self, world: &mut World) {
        let mut menu_event_channel = EventChannel::<MenuEvent<main_menu::Index>>::with_capacity(20);
        let reader_id = menu_event_channel.register_reader();
        self.menu_event_reader.get_or_insert(reader_id);

        world.add_resource(menu_event_channel);
    }

    fn terminate_menu_event_channel(&mut self, _world: &mut World) {
        // By design there is no function to unregister a reader from an `EventChannel`.
        // Nor is there one to remove a resource from the `World`.

        self.menu_event_reader.take();
    }

    fn initialize_menu_items(&mut self, world: &mut World) {
        let (_, font_bold, _, _) = read_fonts(world);

        let mut item_indices = vec![main_menu::Index::StartGame, main_menu::Index::Exit];
        item_indices
            .drain(..)
            .enumerate()
            .for_each(|(order, index)| {
                let mut text_transform = UiTransform::new(
                    index.title().to_string(),
                    20.,
                    order as f32 * 50. + 20.,
                    1.,
                    400.,
                    100.,
                    0,
                );
                let ui_text_size_fn = |_transform: &mut UiTransform, (_width, _height)| {};

                {
                    let dim = world.read_resource::<ScreenDimensions>();
                    ui_text_size_fn(&mut text_transform, (dim.width(), dim.height()));
                }

                let menu_item_entity = world
                    .create_entity()
                    .with(text_transform)
                    .with(UiText::new(
                        font_bold.clone(),
                        index.title().to_string(),
                        [1., 1., 1., 1.],
                        FONT_SIZE,
                    ))
                    .with(UiResize(Box::new(ui_text_size_fn)))
                    .with(MouseReactive)
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

impl amethyst::State for State {
    fn on_start(&mut self, world: &mut World) {
        self.dispatch = Some(ParSeq::new(
            UiEventHandlerSystem::new(),
            world.read_resource::<Arc<rayon::ThreadPool>>().clone(),
        ));

        self.initialize_menu_event_channel(world);
        self.initialize_menu_items(world);
    }

    fn on_stop(&mut self, world: &mut World) {
        self.terminate_menu_items(world);
        self.terminate_menu_event_channel(world);

        self.dispatch.take();
    }

    // Need to explicitly hide and show the menu items during pause and resume
    fn on_resume(&mut self, world: &mut World) {
        self.initialize_menu_items(world);
    }

    fn on_pause(&mut self, world: &mut World) {
        self.terminate_menu_items(world);
    }

    fn update(&mut self, world: &mut World) -> Trans {
        self.dispatch.as_mut().unwrap().dispatch(&world.res);

        let menu_event_channel = world.read_resource::<EventChannel<MenuEvent<main_menu::Index>>>();

        let mut reader_id = self.menu_event_reader
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

type FH = FontHandle;
fn read_fonts(world: &mut World) -> (FH, FH, FH, FH) {
    use application_ui::FontVariant::{Bold, BoldItalic, Italic, Regular};
    (
        world.read_resource_with_id::<FH>(Regular.into()).clone(),
        world.read_resource_with_id::<FH>(Bold.into()).clone(),
        world.read_resource_with_id::<FH>(Italic.into()).clone(),
        world.read_resource_with_id::<FH>(BoldItalic.into()).clone(),
    )
}
