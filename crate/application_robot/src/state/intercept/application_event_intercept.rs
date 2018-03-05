use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use amethyst::shrev::{EventChannel, ReaderId};
use application_input::ApplicationEvent;

use state::Intercept;

/// Reads `ApplicationEvent`s and programmatically controls the application control flow.
#[derive(Debug, Default)]
pub struct ApplicationEventIntercept {
    /// ID of the reader for application events.
    application_event_reader: Option<ReaderId<ApplicationEvent>>,
}

impl ApplicationEventIntercept {
    /// Returns a new ApplicationEventIntercept
    pub fn new() -> Self {
        Default::default()
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

    fn handle_application_events(&mut self, world: &mut World) -> Option<Trans> {
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();

        let mut reader_id = self.application_event_reader
            .as_mut()
            .expect("Expected reader to be set");
        let mut storage_iterator = app_event_channel.read(&mut reader_id);
        if let Some(&ApplicationEvent::Exit) = storage_iterator.next() {
            return Some(Trans::Quit);
        }

        None
    }
}

impl Intercept for ApplicationEventIntercept {
    fn on_start_begin(&mut self, world: &mut World) {
        self.initialize_application_event_reader(world);
    }

    fn handle_event_begin(&mut self, _world: &mut World, event: &mut Event) -> Option<Trans> {
        if let Event::WindowEvent { ref event, .. } = *event {
            match *event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }
                | WindowEvent::Closed => Some(Trans::Quit),
                _ => None,
            }
        } else {
            None
        }
    }

    fn fixed_update_begin(&mut self, world: &mut World) -> Option<Trans> {
        self.handle_application_events(world)
    }

    fn update_begin(&mut self, world: &mut World) -> Option<Trans> {
        self.handle_application_events(world)
    }
}

#[cfg(test)]
mod test {
    use std::mem::discriminant;

    use amethyst::prelude::*;
    use amethyst::renderer::{Event, WindowEvent};
    use amethyst::shrev::EventChannel;
    use application_input::ApplicationEvent;
    use enigo::{Enigo, Key, KeyboardControllable};
    use winit::{ControlFlow, EventsLoop, Window};

    use state::Intercept;
    use super::ApplicationEventIntercept;

    fn setup() -> (ApplicationEventIntercept, World) {
        let mut world = World::new();
        world.add_resource(EventChannel::<ApplicationEvent>::with_capacity(10));

        (ApplicationEventIntercept::new(), world)
    }

    #[test]
    fn on_start_begin_initializes_application_event_channel_reader() {
        let (mut intercept, mut world) = setup();

        assert!(intercept.application_event_reader.is_none());

        intercept.on_start_begin(&mut world);

        assert!(intercept.application_event_reader.is_some());
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();
        let mut reader_id = &mut intercept.application_event_reader.as_mut().unwrap();
        assert_eq!(None, app_event_channel.read(&mut reader_id).next());
    }

    #[test]
    fn fixed_update_begin_returns_trans_quit_on_application_event() {
        let (mut intercept, mut world) = setup();

        // register reader
        intercept.on_start_begin(&mut world);

        {
            let mut app_event_channel = world.write_resource::<EventChannel<ApplicationEvent>>();
            app_event_channel.single_write(ApplicationEvent::Exit);
        } // kcov-ignore

        assert_eq!(
            discriminant(&Trans::Quit),
            discriminant(&intercept.fixed_update_begin(&mut world).unwrap())
        );
    }

    #[test]
    fn update_begin_returns_trans_quit_on_application_event() {
        let (mut intercept, mut world) = setup();

        // register reader
        intercept.on_start_begin(&mut world);

        {
            let mut app_event_channel = world.write_resource::<EventChannel<ApplicationEvent>>();
            app_event_channel.single_write(ApplicationEvent::Exit);
        } // kcov-ignore

        assert_eq!(
            discriminant(&Trans::Quit),
            discriminant(&intercept.update_begin(&mut world).unwrap())
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
    fn handle_event_begin_returns_correct_transition_on_keyboard_input() {
        let (mut intercept, mut world) = setup();

        let mut events_loop = EventsLoop::new();
        let _window = Window::new(&events_loop).unwrap();

        let mut attempts = 3;

        // None on Backspace key
        match_window_event(&mut events_loop, Key::Backspace, |mut event| {
            match intercept.handle_event_begin(&mut world, &mut event) {
                // kcov-ignore-start
                Some(Trans::None) => Some(Err("Expected None but was Some(Trans::None)")),
                Some(Trans::Quit) => Some(Err(
                    "Expected None but was Some(Trans::Quit) on Backspace key",
                )),
                Some(Trans::Pop) => Some(Err("Expected None but was Some(Trans::Pop)")),
                Some(Trans::Push(..)) => Some(Err("Expected None but was Some(Trans::Push(..))")),
                Some(Trans::Switch(..)) => {
                    Some(Err("Expected None but was Some(Trans::Switch(..))"))
                }
                // kcov-ignore-end
                None => Some(Ok(())),
            }
        }); // kcov-ignore

        // Trans::Quit on Escape key
        match_window_event(&mut events_loop, Key::Escape, |mut event| {
            match intercept.handle_event_begin(&mut world, &mut event) {
                Some(Trans::Quit) => Some(Ok(())),
                // kcov-ignore-start
                Some(Trans::None) => {
                    attempts -= 1;
                    if attempts == 0 {
                        Some(Err("Expected Trans::Quit but was Trans::None"))
                    } else {
                        None
                    }
                }
                Some(Trans::Pop) => Some(Err("Expected Trans::Quit but was Trans::Pop")),
                Some(Trans::Push(..)) => Some(Err("Expected Trans::Quit but was Trans::Push(..)")),
                Some(Trans::Switch(..)) => {
                    Some(Err("Expected Trans::Quit but was Trans::Switch(..)"))
                }
                None => Some(Err("Expected Some(Trans::Quit) but was None")), // kcov-ignore-end
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
