use std::sync::Arc;

use amethyst;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use amethyst::shred::ParSeq;
use amethyst::shrev::{EventChannel, ReaderId};
use application_input::ApplicationEvent;
use application_menu::MenuEvent;
use rayon;

use index::Index;
use menu_build_fn::MenuBuildFn;
use system::UiEventHandlerSystem;

/// Game mode selection state.
///
/// Available transitions:
///
/// * Select game mode.
/// * Exit application.
#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct State {
    /// Dispatcher for UI handler system.
    #[derivative(Debug = "ignore")]
    dispatch: Option<ParSeq<Arc<rayon::ThreadPool>, UiEventHandlerSystem>>,
    /// ID of the reader for application events.
    application_event_reader: Option<ReaderId<ApplicationEvent>>,
    /// Function used to build the menu.
    menu_build_fn: MenuBuildFn,
    /// Menu item entities, which we create / delete when the state is run / paused
    menu_items: Vec<Entity>,
    /// ID of the reader for menu events.
    menu_event_reader: Option<ReaderId<MenuEvent<Index>>>,
}

impl State {
    /// Returns a new game mode menu state.
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(test)]
    fn internal_new(menu_build_fn: MenuBuildFn) -> Self {
        State {
            dispatch: Option::default(),
            application_event_reader: Option::default(),
            menu_build_fn,
            menu_items: Vec::default(),
            menu_event_reader: Option::default(),
        } // kcov-ignore
    }

    fn initialize_dispatcher(&mut self, world: &mut World) {
        self.dispatch = Some(ParSeq::new(
            UiEventHandlerSystem::new(),
            world.read_resource::<Arc<rayon::ThreadPool>>().clone(),
        ));
    }

    fn terminate_dispatcher(&mut self) {
        self.dispatch.take();
    }

    fn initialize_application_event_reader(&mut self, world: &mut World) {
        // You can't (don't have to) unregister a reader from an EventChannel in `on_stop();`:
        //
        // > @torkleyy: No need to unregister, it's just two integer values.
        // > @Rhuagh: Just drop the reader id
        let reader_id = world
            .write_resource::<EventChannel<ApplicationEvent>>()
            .register_reader(); // kcov-ignore

        self.application_event_reader.get_or_insert(reader_id);
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

impl amethyst::State for State {
    fn on_start(&mut self, world: &mut World) {
        self.initialize_dispatcher(world);
        self.initialize_application_event_reader(world);
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

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        // Intentionally ignore testing mouse events as we cannot guarantee the cursor is over the
        // window when someone runs `cargo test`
        // kcov-ignore-start-mouse
        match event {
            // kcov-ignore-end-mouse
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }
                | WindowEvent::Closed => Trans::Quit, // kcov-ignore-mouse
                _ => Trans::None,
            },
            _ => Trans::None, // kcov-ignore-mouse
        }
    }

    fn update(&mut self, world: &mut World) -> Trans {
        {
            self.dispatch.as_mut().unwrap().dispatch(&mut world.res);
        }

        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();

        let mut reader_id = self.application_event_reader
            .as_mut()
            .expect("Expected reader to be set");
        let mut storage_iterator = app_event_channel.read(&mut reader_id);
        if let Some(_event) = storage_iterator.next() {
            return Trans::Quit;
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

    use amethyst::ecs::World;
    use amethyst::prelude::*;
    use amethyst::renderer::{Event, WindowEvent};
    use amethyst::shrev::EventChannel;
    use amethyst::State as AmethystState;
    use amethyst::ui::UiEvent;
    use application_input::ApplicationEvent;
    use application_menu::{MenuEvent, MenuItem};
    use enigo::{Enigo, Key, KeyboardControllable};
    use rayon_core::ThreadPoolBuilder;
    use winit::{ControlFlow, EventsLoop, Window};

    use menu_build_fn::MenuBuildFn;
    use index::Index;
    use super::State;

    fn setup() -> (State, World) {
        setup_with_menu_items(MenuBuildFn(Box::new(|_, _| {})))
    }

    fn setup_with_menu_items(menu_build_fn: MenuBuildFn) -> (State, World) {
        let mut world = World::new();
        // TODO: use rayon::ThreadPoolBuilder; https://github.com/amethyst/amethyst/pull/579
        world.add_resource(Arc::new(ThreadPoolBuilder::new().build().unwrap()));
        world.add_resource(EventChannel::<ApplicationEvent>::with_capacity(10));
        world.add_resource(EventChannel::<MenuEvent<Index>>::with_capacity(10));
        world.add_resource(EventChannel::<UiEvent>::with_capacity(10)); // needed by system
        world.register::<MenuItem<Index>>();

        (State::internal_new(menu_build_fn), world)
    }

    #[test]
    fn on_start_initializes_dispatcher() {
        let (mut state, mut world) = setup();

        assert!(state.dispatch.is_none());

        state.on_start(&mut world);

        assert!(state.dispatch.is_some());
    }

    #[test]
    fn on_start_initializes_application_event_channel_reader() {
        let (mut state, mut world) = setup();

        assert!(state.application_event_reader.is_none());

        state.on_start(&mut world);

        assert!(state.application_event_reader.is_some());
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();
        let mut reader_id = &mut state.application_event_reader.as_mut().unwrap();
        assert_eq!(None, app_event_channel.read(&mut reader_id).next());
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
    fn update_returns_trans_quit_on_application_event() {
        let (mut state, mut world) = setup();

        // register reader
        state.on_start(&mut world);

        {
            let mut app_event_channel = world.write_resource::<EventChannel<ApplicationEvent>>();
            app_event_channel.single_write(ApplicationEvent::Exit);
        } // kcov-ignore

        assert_eq!(
            discriminant(&Trans::Quit),
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

    /// We have to run multiple tests in a single function because:
    ///
    /// * Test functions are run in parallel.
    /// * We _could_ use a mutex to prevent windowed tests from running simultaneously, as input
    ///   generated from the `Enigo` library is transmitted operating system wide. However, this
    ///   does not work because of the next point.
    /// * When two windowed tests run sequentially, the test executable will fail in one of the
    ///   following modes:
    ///
    ///     - An incorrectly failing test: `Expected Trans::None but was Trans::Quit`
    ///     - Illegal application signal: `(signal: 4, SIGILL: illegal instruction)`
    ///
    ///         This is possibly a bug in `winit`, but there is not enough information to conclude
    ///         that this is true. I've tried both `winit` 0.10.0 and `0.11.0` (latest at time of
    ///         writing).
    ///
    ///         Possibly this bug: https://github.com/tomaka/winit/issues/347
    ///
    /// I have tried using `thread::sleep`s to check if it is a race condition between opening the
    /// window and the programmatic input, but it does not appear to be the case - it fails in the
    /// same way even with the `sleep`s.
    ///
    /// On Ubuntu 17.10, the window that starts up during the test does not get destroyed until
    /// after the entire test executable ends, even if you `drop(window)` and `drop(events_loop)`.
    /// This may be a symptom of the problem.
    #[test]
    fn handle_event_returns_correct_transition_on_keyboard_input() {
        let (mut state, mut world) = setup();

        let mut events_loop = EventsLoop::new();
        let _window = Window::new(&events_loop).unwrap();

        let mut attempts = 3;

        // Trans::None on Backspace key
        match_window_event(&mut events_loop, Key::Backspace, |event| {
            match state.handle_event(&mut world, event) {
                Trans::None => Some(Ok(())),
                // kcov-ignore-start
                Trans::Quit => Some(Err(
                    "Expected Trans::None but was Trans::Quit on Backspace key",
                )),
                Trans::Pop => Some(Err("Expected Trans::None but was Trans::Pop")),
                Trans::Push(..) => Some(Err("Expected Trans::None but was Trans::Push(..)")),
                Trans::Switch(..) => Some(Err("Expected Trans::None but was Trans::Switch(..)")),
                // kcov-ignore-end
            }
        }); // kcov-ignore

        // Trans::Quit on Escape key
        match_window_event(&mut events_loop, Key::Escape, |event| {
            match state.handle_event(&mut world, event) {
                Trans::Quit => Some(Ok(())),
                // kcov-ignore-start
                Trans::None => {
                    attempts -= 1;
                    if attempts == 0 {
                        Some(Err("Expected Trans::Quit but was Trans::None"))
                    } else {
                        None
                    }
                }
                Trans::Pop => Some(Err("Expected Trans::Quit but was Trans::Pop")),
                Trans::Push(..) => Some(Err("Expected Trans::Quit but was Trans::Push(..)")),
                Trans::Switch(..) => Some(Err("Expected Trans::Quit but was Trans::Switch(..)")),
                // kcov-ignore-end
            }
        });
    } // kcov-ignore

    fn match_window_event<F>(events_loop: &mut EventsLoop, key: Key, mut assertion_fn: F)
    where
        F: FnMut(Event) -> Option<Result<(), &'static str>>,
    {
        let mut enigo = Enigo::new();
        enigo.key_click(key);

        let mut test_result = None;

        events_loop.run_forever(|event| {
            let result_opt = match event {
                Event::WindowEvent {
                    event: ref window_event,
                    ..
                } => match window_event {
                    &WindowEvent::KeyboardInput { .. } => assertion_fn(event.clone()),
                    _ => None,
                },
                _ => None,
            };
            match result_opt {
                None => ControlFlow::Continue,
                Some(result) => {
                    test_result = Some(result);
                    ControlFlow::Break
                }
            }
        });

        events_loop.poll_events(|_event| {}); // empty event queue

        test_result
            .unwrap()
            .unwrap_or_else(|failure: &str| panic!(failure)); // kcov-ignore
    }
}
