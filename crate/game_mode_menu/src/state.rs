use amethyst;
use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use amethyst::shrev::{EventChannel, ReaderId};
use application_input::ApplicationEvent;

/// Game mode selection state.
///
/// Available transitions:
///
/// * Selection of game mode
/// * Exiting the application
///
#[derive(Debug, Default)]
pub struct State {
    /// ID of the reader for application events.
    application_event_reader: Option<ReaderId<ApplicationEvent>>,
}

impl State {
    /// Returns a new game mode menu state.
    pub fn new() -> Self {
        Default::default()
    }
}

impl amethyst::State for State {
    fn on_start(&mut self, world: &mut World) {
        // You can't unregister a reader from an EventChannel in on_stop because we don't have to
        //
        // @torkleyy: No need to unregister, it's just two integer values.
        // @Rhuagh: Just drop the reader id
        let reader_id = world
            .write_resource::<EventChannel<ApplicationEvent>>()
            .register_reader();

        self.application_event_reader.get_or_insert(reader_id);
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
                }
                | WindowEvent::Closed => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn update(&mut self, world: &mut World) -> Trans {
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();

        let mut reader_id = self.application_event_reader
            .as_mut()
            .expect("Expected reader to be set");
        let mut storage_iterator = app_event_channel.read(&mut reader_id);
        while let Some(_event) = storage_iterator.next() {
            return Trans::Quit;
        }

        Trans::None
    }
}

#[cfg(test)]
mod test {
    use std::panic;
    use std::sync::Mutex;

    use amethyst::ecs::World;
    use amethyst::renderer::{Event, WindowEvent};
    use amethyst::shrev::EventChannel;
    use amethyst::State as AmethystState;
    use amethyst::prelude::*;
    use application_input::ApplicationEvent;
    use enigo::{Enigo, Key, KeyboardControllable};
    use winit::{ControlFlow, EventsLoop, Window};

    use super::State;

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
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
    /// I have tried using `thread::sleep`s to try and see if it is a race condition between opening
    /// the window and the programmatic input, but it does not appear to be the case - it fails in
    /// the same way even with the `sleep`s.
    ///
    /// On Ubuntu 17.10, the window that starts up during the test does not get destroyed until
    /// after the entire test executable ends, even if you `drop(window)` and `drop(events_loop)`.
    /// This may be a symptom of the problem.
    macro_rules! test {
        (fn $name:ident() $body:block) => {
            #[test]
            fn $name() {
                let guard = TEST_MUTEX.lock().unwrap();

                // A `panic!` indicates a failing test. However this would normally poison the mutex
                // used to serialize tests. Therefore we capture the test result using a `Result`,
                // free the mutex, then throw the `panic!` if it was a failure.
                let execution_result = panic::catch_unwind(|| -> Result<(), &str> {
                    { $body }
                });
                drop(guard);
                match execution_result {
                    Ok(test_result) => test_result.unwrap_or_else(|failure: &str| {
                        panic!(failure)
                    }),
                    Err(e) => panic::resume_unwind(e), // kcov-ignore
                }
            }
        }
    }

    fn setup() -> (State, World) {
        let mut world = World::new();
        world.add_resource(EventChannel::<ApplicationEvent>::with_capacity(10));

        (State::new(), world)
    }

    #[test]
    fn on_start_registers_reader() {
        let (mut state, mut world) = setup();

        state.on_start(&mut world);

        assert!(state.application_event_reader.is_some());
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();
        let mut reader_id = &mut state.application_event_reader.as_mut().unwrap();
        assert_eq!(None, app_event_channel.read(&mut reader_id).next());
    }

    test! {
        fn handle_event_returns_trans_quit_on_escape_keyboard_input() {
            let (mut state, mut world) = setup();

            let mut attempts = 3;
            match_window_event(Key::Escape, |event| {
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
            })
        }
    }

    test! {
        fn handle_event_returns_trans_none_on_other_keyboard_input() {
            let (mut state, mut world) = setup();

            match_window_event(Key::Backspace, |event| {
                match state.handle_event(&mut world, event) {
                    Trans::None => Some(Ok(())),
                    // kcov-ignore-begin
                    Trans::Quit => None, // Some(Err("Expected Trans::None but was Trans::Quit"))
                    Trans::Pop => Some(Err("Expected Trans::None but was Trans::Pop")),
                    Trans::Push(..) => Some(Err("Expected Trans::None but was Trans::Push(..)")),
                    Trans::Switch(..) => Some(Err("Expected Trans::None but was Trans::Switch(..)")),
                    // kcov-ignore-end
                }
            })
        }
    }

    fn match_window_event<'f, F>(key: Key, mut assertion_fn: F) -> Result<(), &'f str>
    where
        F: FnMut(Event) -> Option<Result<(), &'f str>>,
    {
        let mut events_loop = EventsLoop::new();
        let _window = Window::new(&events_loop).unwrap();

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

        test_result.unwrap()
    }
}
