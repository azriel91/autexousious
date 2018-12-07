use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::VirtualKeyCode;

use crate::state::Intercept;

/// Exits when the `Escape` key is pressed.
#[derive(Clone, Debug, Default)]
pub struct KeyboardEscapeIntercept;

impl<T> Intercept<T, StateEvent> for KeyboardEscapeIntercept {
    fn handle_event_begin(
        &mut self,
        _data: &mut StateData<'_, T>,
        event: &mut StateEvent,
    ) -> Option<Trans<T, StateEvent>> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Some(Trans::Quit)
            } else {
                None
            }
        } else {
            // TODO: cover this case when there is support for dummy events #15
            None // kcov-ignore
        }
    }
}

#[cfg(test)]
#[cfg(all(windows, feature = "test_ui"))]
mod test {
    use amethyst::prelude::*;
    use amethyst::renderer::{Event, WindowEvent};
    use debug_util_amethyst::assert_eq_opt_trans;
    use enigo::{Enigo, Key, KeyboardControllable};
    use winit::{ControlFlow, EventsLoop, Window};

    use super::KeyboardEscapeIntercept;
    use state::Intercept;

    fn setup() -> (KeyboardEscapeIntercept, World) {
        (KeyboardEscapeIntercept, World::new())
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
    ///
    /// On Windows, this test works if run interactively. However it does not when run through a
    /// Gitlab CI runner.
    #[test]
    fn handle_event_begin_returns_correct_transition_on_keyboard_input() {
        let (mut intercept, mut world) = setup();

        let mut events_loop = EventsLoop::new();
        let _window = Window::new(&events_loop).unwrap();

        let mut attempts = 3;

        // None on Backspace key
        match_window_event(&mut events_loop, Key::Backspace, |mut event| {
            let trans = intercept.handle_event_begin(&mut world, &mut event);
            if trans.is_none() {
                return Some(Ok(()));
            }
            assert_eq_opt_trans(None, trans.as_ref()); // kcov-ignore
            unreachable!(); // kcov-ignore
        }); // kcov-ignore

        // Trans::Quit on Escape key
        match_window_event(&mut events_loop, Key::Escape, |mut event| {
            let trans = intercept.handle_event_begin(&mut world, &mut event);
            if let Some(Trans::Quit) = trans {
                return Some(Ok(()));
            }
            // kcov-ignore-start
            attempts -= 1;
            if attempts == 0 {
                assert_eq_opt_trans(Some(Trans::Quit).as_ref(), trans.as_ref());
            }
            None
            // kcov-ignore-end
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
