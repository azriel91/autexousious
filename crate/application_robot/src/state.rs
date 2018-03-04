use std::fmt::Debug;

use amethyst;
use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use amethyst::shrev::{EventChannel, ReaderId};
use application_input::ApplicationEvent;

/// Wraps a delegate state with automation capabilities.
#[derive(Debug)]
pub struct State<S: amethyst::State + Debug> {
    /// ID of the reader for application events.
    application_event_reader: Option<ReaderId<ApplicationEvent>>,
    /// State to delegate behaviour to.
    delegate: Box<S>,
}

impl<S: amethyst::State + Debug> State<S> {
    /// Returns a new application robot state.
    pub fn new(delegate: S) -> Self {
        State {
            application_event_reader: Option::default(),
            delegate: Box::new(delegate),
        } // kcov-ignore
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

impl<S: amethyst::State + Debug> amethyst::State for State<S> {
    fn on_start(&mut self, world: &mut World) {
        self.initialize_application_event_reader(world);
        self.delegate.on_start(world);
    }

    fn on_stop(&mut self, world: &mut World) {
        self.delegate.on_stop(world);
    }

    fn on_pause(&mut self, world: &mut World) {
        self.delegate.on_pause(world);
    }

    fn on_resume(&mut self, world: &mut World) {
        self.delegate.on_resume(world);
    }

    fn handle_event(&mut self, world: &mut World, event: Event) -> Trans {
        if let Event::WindowEvent { ref event, .. } = event {
            match *event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }
                | WindowEvent::Closed => return Trans::Quit, // kcov-ignore-mouse
                _ => {}
            }
        }

        self.delegate.handle_event(world, event)
    }

    fn fixed_update(&mut self, world: &mut World) -> Trans {
        if let Some(trans) = self.handle_application_events(world) {
            return trans;
        }

        self.delegate.fixed_update(world)
    }

    fn update(&mut self, world: &mut World) -> Trans {
        if let Some(trans) = self.handle_application_events(world) {
            return trans;
        }

        self.delegate.update(world)
    }
}

#[cfg(test)]
mod test {
    use std::mem::discriminant;

    use amethyst;
    use amethyst::ecs::World;
    use amethyst::prelude::*;
    use amethyst::renderer::{Event, WindowEvent};
    use amethyst::shrev::EventChannel;
    use amethyst::State as AmethystState;
    use application_input::ApplicationEvent;
    use enigo::{Enigo, Key, KeyboardControllable};
    use winit::{ControlFlow, EventsLoop, Window};

    use super::State;

    fn setup() -> (State<MockState>, World) {
        let mut world = World::new();
        world.add_resource(EventChannel::<ApplicationEvent>::with_capacity(10));

        (State::new(MockState::new()), world)
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
    fn on_start_delegates_to_delegate() {
        let (mut state, mut world) = setup();

        state.on_start(&mut world);

        assert_eq!(vec![Invocation::OnStart], state.delegate.invocations);
    }

    #[test]
    fn on_stop_delegates_to_delegate() {
        let (mut state, mut world) = setup();

        state.on_stop(&mut world);

        assert_eq!(vec![Invocation::OnStop], state.delegate.invocations);
    }

    #[test]
    fn on_pause_delegates_to_delegate() {
        let (mut state, mut world) = setup();

        state.on_pause(&mut world);

        assert_eq!(vec![Invocation::OnPause], state.delegate.invocations);
    }

    #[test]
    fn on_resume_delegates_to_delegate() {
        let (mut state, mut world) = setup();

        state.on_resume(&mut world);

        assert_eq!(vec![Invocation::OnResume], state.delegate.invocations);
    }

    #[test]
    fn fixed_update_delegates_to_delegate() {
        let (mut state, mut world) = setup();

        // register reader
        state.on_start(&mut world);

        state.fixed_update(&mut world);

        assert_eq!(
            vec![Invocation::OnStart, Invocation::FixedUpdate],
            state.delegate.invocations
        );
    }

    #[test]
    fn update_delegates_to_delegate() {
        let (mut state, mut world) = setup();

        // register reader
        state.on_start(&mut world);

        state.update(&mut world);

        assert_eq!(
            vec![Invocation::OnStart, Invocation::Update],
            state.delegate.invocations
        );
    }

    #[test]
    fn fixed_update_returns_trans_quit_on_application_event() {
        let (mut state, mut world) = setup();

        // register reader
        state.on_start(&mut world);

        {
            let mut app_event_channel = world.write_resource::<EventChannel<ApplicationEvent>>();
            app_event_channel.single_write(ApplicationEvent::Exit);
        } // kcov-ignore

        assert_eq!(
            discriminant(&Trans::Quit),
            discriminant(&state.fixed_update(&mut world))
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

        // Check for delegation of handle_event
        assert_eq!(vec![Invocation::HandleEvent], state.delegate.invocations);

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

    #[derive(Debug, PartialEq)]
    enum Invocation {
        OnStart,
        OnStop,
        OnPause,
        OnResume,
        HandleEvent,
        FixedUpdate,
        Update,
    }

    #[derive(Debug, Default)]
    struct MockState {
        invocations: Vec<Invocation>,
    }

    impl MockState {
        fn new() -> Self {
            Default::default()
        }
    }

    impl amethyst::State for MockState {
        fn on_start(&mut self, _: &mut World) {
            self.invocations.push(Invocation::OnStart);
        }

        fn on_stop(&mut self, _: &mut World) {
            self.invocations.push(Invocation::OnStop);
        }

        fn on_pause(&mut self, _: &mut World) {
            self.invocations.push(Invocation::OnPause);
        }

        fn on_resume(&mut self, _: &mut World) {
            self.invocations.push(Invocation::OnResume);
        }

        fn handle_event(&mut self, _: &mut World, _: Event) -> Trans {
            self.invocations.push(Invocation::HandleEvent);
            Trans::None
        }

        fn fixed_update(&mut self, _: &mut World) -> Trans {
            self.invocations.push(Invocation::FixedUpdate);
            Trans::None
        }

        fn update(&mut self, _: &mut World) -> Trans {
            self.invocations.push(Invocation::Update);
            Trans::None
        }
    }
}
