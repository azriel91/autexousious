use std::sync::Arc;

use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::shred::ParSeq;
use amethyst::shrev::{EventChannel, ReaderId};
use application_menu::MenuEvent;
use rayon;

use Index;
use MenuBuildFn;
use UiEventHandlerSystem;

/// Game mode selection state.
///
/// Available transitions:
///
/// * Select game mode.
/// * Exit application.
#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct GameModeMenuState {
    /// Dispatcher for UI handler system.
    #[derivative(Debug = "ignore")]
    dispatch: Option<ParSeq<Arc<rayon::ThreadPool>, UiEventHandlerSystem>>,
    /// Function used to build the menu.
    menu_build_fn: MenuBuildFn,
    /// Menu item entities, which we create / delete when the state is run / paused
    menu_items: Vec<Entity>,
    /// ID of the reader for menu events.
    menu_event_reader: Option<ReaderId<MenuEvent<Index>>>,
}

impl GameModeMenuState {
    /// Returns a new game mode menu state.
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(test)]
    fn internal_new(menu_build_fn: MenuBuildFn) -> Self {
        GameModeMenuState {
            menu_build_fn,
            ..Default::default()
        } // kcov-ignore
    }

    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatch = ParSeq::new(
            UiEventHandlerSystem::new(),
            world.read_resource::<Arc<rayon::ThreadPool>>().clone(),
        );
        dispatch.setup(&mut world.res);
        self.dispatch = Some(dispatch);
    }

    fn terminate_dispatcher(&mut self) {
        self.dispatch.take();
    }

    fn initialize_menu_event_channel(&mut self, world: &mut World) {
        let mut menu_event_channel = EventChannel::<MenuEvent<Index>>::with_capacity(20);
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
        // https://github.com/rust-lang/rust/issues/26186
        // https://stackoverflow.com/q/46472082/1576773
        (&mut *self.menu_build_fn)(world, &mut self.menu_items);
    }

    fn terminate_menu_items(&mut self, world: &mut World) {
        self.menu_items.drain(..).for_each(|menu_item| {
            world
                .delete_entity(menu_item)
                .expect("Failed to delete menu item.");
        });
    }
}

impl State for GameModeMenuState {
    fn on_start(&mut self, world: &mut World) {
        self.initialize_dispatcher(world);
        self.initialize_menu_event_channel(world);
        self.initialize_menu_items(world);
    }

    fn on_stop(&mut self, world: &mut World) {
        self.terminate_menu_items(world);
        self.terminate_menu_event_channel(world);
        self.terminate_dispatcher();
    }

    // Need to explicitly hide and show the menu items during pause and resume
    fn on_resume(&mut self, world: &mut World) {
        self.initialize_menu_items(world);
    }

    fn on_pause(&mut self, world: &mut World) {
        self.terminate_menu_items(world);
    }

    fn update(&mut self, world: &mut World) -> Trans {
        {
            self.dispatch.as_mut().unwrap().dispatch(&world.res);
        }

        let menu_event_channel = world.read_resource::<EventChannel<MenuEvent<Index>>>();

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

#[cfg(test)]
mod test {
    use std::mem::discriminant;
    use std::sync::Arc;

    use amethyst::ecs::prelude::World;
    use amethyst::prelude::*;
    use amethyst::shrev::EventChannel;
    use amethyst::ui::UiEvent;
    use application_menu::{MenuEvent, MenuItem};
    use rayon::ThreadPoolBuilder;

    use super::GameModeMenuState;
    use index::Index;
    use menu_build_fn::MenuBuildFn;

    fn setup() -> (GameModeMenuState, World) {
        setup_with_menu_items(MenuBuildFn(Box::new(|_, _| {})))
    }

    fn setup_with_menu_items(menu_build_fn: MenuBuildFn) -> (GameModeMenuState, World) {
        let mut world = World::new();
        world.add_resource(Arc::new(ThreadPoolBuilder::new().build().unwrap()));
        world.add_resource(EventChannel::<MenuEvent<Index>>::with_capacity(10));
        world.add_resource(EventChannel::<UiEvent>::with_capacity(10)); // needed by system
        world.register::<MenuItem<Index>>();

        (GameModeMenuState::internal_new(menu_build_fn), world)
    }

    #[test]
    fn on_start_initializes_dispatcher() {
        let (mut state, mut world) = setup();

        assert!(state.dispatch.is_none());

        state.on_start(&mut world);

        assert!(state.dispatch.is_some());
    }

    #[test]
    fn on_start_initializes_menu_event_channel_reader() {
        let (mut state, mut world) = setup();

        assert!(state.menu_event_reader.is_none());

        state.on_start(&mut world);

        assert!(state.menu_event_reader.is_some());
        let menu_event_channel = world.read_resource::<EventChannel<MenuEvent<Index>>>();
        let mut reader_id = &mut state.menu_event_reader.as_mut().unwrap();
        assert_eq!(None, menu_event_channel.read(&mut reader_id).next());
    }

    #[test]
    fn on_start_initializes_menu_items() {
        let (mut state, mut world) = setup_with_menu_items(MenuBuildFn(Box::new(
            |world, menu_items| menu_items.push(world.create_entity().build()),
        )));

        assert!(state.menu_items.is_empty());

        state.on_start(&mut world);

        assert_eq!(1, state.menu_items.len());
    }

    #[test]
    fn on_stop_terminates_dispatcher() {
        let (mut state, mut world) = setup();

        state.on_start(&mut world);

        assert!(state.dispatch.is_some());

        state.on_stop(&mut world);

        assert!(state.dispatch.is_none());
    }

    #[test]
    fn on_stop_terminates_menu_event_channel_reader() {
        let (mut state, mut world) = setup();

        state.on_start(&mut world);

        assert!(state.menu_event_reader.is_some());

        state.on_stop(&mut world);

        assert!(state.menu_event_reader.is_none());
    }

    #[test]
    fn on_stop_terminates_menu_items() {
        let (mut state, mut world) = setup_with_menu_items(MenuBuildFn(Box::new(
            |world, menu_items| menu_items.push(world.create_entity().build()),
        )));

        state.on_start(&mut world);

        assert_eq!(1, state.menu_items.len());

        state.on_stop(&mut world);

        assert!(state.menu_items.is_empty());
    }

    #[test]
    fn on_pause_terminates_menu_items() {
        let (mut state, mut world) = setup_with_menu_items(MenuBuildFn(Box::new(
            |world, menu_items| menu_items.push(world.create_entity().build()),
        )));

        state.on_start(&mut world);

        assert_eq!(1, state.menu_items.len());

        state.on_pause(&mut world);

        assert!(state.menu_items.is_empty());
    }

    #[test]
    fn on_resume_initializes_menu_items() {
        let (mut state, mut world) = setup_with_menu_items(MenuBuildFn(Box::new(
            |world, menu_items| menu_items.push(world.create_entity().build()),
        )));

        state.on_start(&mut world);

        assert_eq!(1, state.menu_items.len());

        state.on_pause(&mut world);

        assert!(state.menu_items.is_empty());

        state.on_resume(&mut world);

        assert_eq!(1, state.menu_items.len());
    }

    #[test]
    fn update_returns_trans_none_when_no_application_or_menu_event_exists() {
        let (mut state, mut world) = setup();

        // register reader
        state.on_start(&mut world);

        assert_eq!(
            discriminant(&Trans::None),
            discriminant(&state.update(&mut world))
        );
    }

    #[test]
    fn update_returns_trans_quit_on_close_menu_event() {
        let (mut state, mut world) = setup();

        // register reader
        state.on_start(&mut world);

        {
            let mut menu_event_channel = world.write_resource::<EventChannel<MenuEvent<Index>>>();
            menu_event_channel.single_write(MenuEvent::Close);
        } // kcov-ignore

        assert_eq!(
            discriminant(&Trans::Quit),
            discriminant(&state.update(&mut world))
        );
    }
}
