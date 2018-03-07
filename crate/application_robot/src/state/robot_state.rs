use std::fmt::Debug;

use amethyst::prelude::*;
use amethyst::renderer::Event;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

use state::intercept::ApplicationEventIntercept;
use state::Intercept;

/// Wraps a delegate state with automation capabilities.
#[derive(Builder, Debug)]
#[builder(pattern = "owned", derive(Debug))]
pub struct RobotState<S: State + Debug> {
    /// Intercepts to track and control application behaviour.
    ///
    /// Box<Intercept> is a trait object, which does not implement Sized, needed by the generated
    /// setter from the `Builder` derive, so we instead provide default intercepts, and functions
    /// to toggle the enablement of certain `Intercept`s.
    #[builder(default = "self.default_intercepts()?")]
    #[builder(setter(skip))]
    intercepts: Vec<Box<Intercept>>,
    /// State to delegate behaviour to.
    delegate: Box<S>,
}

impl<S: State + Debug> RobotStateBuilder<S> {
    fn default_intercepts(&self) -> Result<Vec<Box<Intercept>>, String> {
        Ok(vec![Box::new(ApplicationEventIntercept::new())])
    }
}

impl<S: State + Debug> RobotState<S> {
    /// Returns a new application robot state.
    pub fn new(delegate: S) -> Self {
        RobotState {
            intercepts: Default::default(),
            delegate: Box::new(delegate),
        } // kcov-ignore
    }

    fn fold_trans_begin<F>(
        &mut self,
        initial_trans: Option<Trans>,
        mut intercept_fn: F,
    ) -> Option<Trans>
    where
        F: FnMut(&mut Box<Intercept>) -> Option<Trans>,
    {
        self.intercepts
            .iter_mut()
            .fold_while(initial_trans, |trans, intercept| {
                if trans.is_none() {
                    Continue(intercept_fn(intercept))
                } else {
                    Done(trans)
                }
            })
            .into_inner()
    }

    fn fold_trans_end<F>(
        &mut self,
        initial_trans: Option<Trans>,
        mut intercept_fn: F,
    ) -> Option<Trans>
    where
        F: FnMut(&mut Box<Intercept>, &Trans) -> Option<Trans>,
    {
        self.intercepts
            .iter_mut()
            .fold_while(initial_trans, |trans, intercept| {
                if trans.is_none() {
                    Continue(intercept_fn(intercept, trans.as_ref().unwrap()))
                } else {
                    Done(trans)
                }
            })
            .into_inner()
    }
}

impl<S: State + Debug> State for RobotState<S> {
    fn on_start(&mut self, world: &mut World) {
        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.on_start_begin(world));

        self.delegate.on_start(world);

        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.on_start_end(world));
    }

    fn on_stop(&mut self, world: &mut World) {
        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.on_stop_begin(world));

        self.delegate.on_stop(world);

        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.on_stop_end(world));
    }

    fn on_pause(&mut self, world: &mut World) {
        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.on_pause_begin(world));

        self.delegate.on_pause(world);

        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.on_pause_end(world));
    }

    fn on_resume(&mut self, world: &mut World) {
        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.on_resume_begin(world));

        self.delegate.on_resume(world);

        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.on_resume_end(world));
    }

    fn handle_event(&mut self, world: &mut World, mut event: Event) -> Trans {
        let intercept_trans = self.fold_trans_begin(None, |intercept| {
            intercept.handle_event_begin(world, &mut event)
        });

        let trans = intercept_trans.or_else(|| Some(self.delegate.handle_event(world, event)));

        self.fold_trans_end(trans, |intercept, trans| {
            intercept.handle_event_end(world, trans)
        }).unwrap()
    }

    fn fixed_update(&mut self, world: &mut World) -> Trans {
        let intercept_trans =
            self.fold_trans_begin(None, |intercept| intercept.fixed_update_begin(world));

        let trans = intercept_trans.or_else(|| Some(self.delegate.fixed_update(world)));

        self.fold_trans_end(trans, |intercept, trans| {
            intercept.fixed_update_end(world, trans)
        }).unwrap()
    }

    fn update(&mut self, world: &mut World) -> Trans {
        let intercept_trans =
            self.fold_trans_begin(None, |intercept| intercept.update_begin(world));

        let trans = intercept_trans.or_else(|| Some(self.delegate.update(world)));

        self.fold_trans_end(trans, |intercept, trans| intercept.update_end(world, trans))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use amethyst::ecs::World;
    use amethyst::prelude::*;
    use amethyst::renderer::{Event, WindowEvent};
    use enigo::{Enigo, Key, KeyboardControllable};
    use winit::{ControlFlow, EventsLoop, Window};

    use state::Intercept;
    use super::{RobotState, RobotStateBuilder};

    fn setup(
        invocations: Rc<RefCell<Vec<Invocation>>>,
        intercepts: Vec<Box<Intercept>>,
    ) -> (RobotState<MockState>, World) {
        let mut robot_state = RobotStateBuilder::default()
                .delegate(Box::new(MockState::new(invocations.clone())))
                // .intercepts(vec![Box::new(MockIntercept(None))])
                .build()
                .expect("Failed to build RobotState");

        // TODO: Use setter method, pending <https://gitlab.com/azriel91/autexousious/issues/17>
        robot_state.intercepts = intercepts;

        let world = World::new();

        (robot_state, world)
    }

    fn setup_without_intercepts() -> (RobotState<MockState>, World) {
        setup(Rc::new(RefCell::new(vec![])), Vec::new())
    }

    #[macro_use]
    macro_rules! delegate_test {
        ($test_name:ident, $function:ident, $invocation:ident) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world) = setup_without_intercepts();

                state.$function(&mut world);

                assert_eq!(
                    vec![Invocation::$invocation],
                    *state.delegate.invocations.borrow()
                );
            }
        }
    }

    delegate_test!(on_start_delegates_to_state, on_start, OnStart);
    delegate_test!(on_stop_delegates_to_state, on_stop, OnStop);
    delegate_test!(on_pause_delegates_to_state, on_pause, OnPause);
    delegate_test!(on_resume_delegates_to_state, on_resume, OnResume);
    delegate_test!(fixed_update_delegates_to_state, fixed_update, FixedUpdate);
    delegate_test!(update_delegates_to_state, update, Update);

    // TODO: We ignore running this test because we cannot construct a window in both this test and
    // in the application_event_intercept module due to
    // <https://gitlab.com/azriel91/autexousious/issues/16>.
    #[test]
    #[ignore]
    fn handle_event_delegates_to_state() {
        let (mut state, mut world) = setup_without_intercepts();

        let mut events_loop = EventsLoop::new();
        let _window = Window::new(&events_loop).unwrap();
        let event = get_window_event(&mut events_loop);

        state.handle_event(&mut world, event);

        assert_eq!(
            vec![Invocation::HandleEvent],
            *state.delegate.invocations.borrow()
        );
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
        invocations: Rc<RefCell<Vec<Invocation>>>,
    }

    impl MockState {
        fn new(invocations: Rc<RefCell<Vec<Invocation>>>) -> Self {
            MockState { invocations }
        }
    }

    impl State for MockState {
        fn on_start(&mut self, _: &mut World) {
            self.invocations.borrow_mut().push(Invocation::OnStart);
        }

        fn on_stop(&mut self, _: &mut World) {
            self.invocations.borrow_mut().push(Invocation::OnStop);
        }

        fn on_pause(&mut self, _: &mut World) {
            self.invocations.borrow_mut().push(Invocation::OnPause);
        }

        fn on_resume(&mut self, _: &mut World) {
            self.invocations.borrow_mut().push(Invocation::OnResume);
        }

        fn handle_event(&mut self, _: &mut World, _: Event) -> Trans {
            self.invocations.borrow_mut().push(Invocation::HandleEvent);
            Trans::None
        }

        fn fixed_update(&mut self, _: &mut World) -> Trans {
            self.invocations.borrow_mut().push(Invocation::FixedUpdate);
            Trans::None
        }

        fn update(&mut self, _: &mut World) -> Trans {
            self.invocations.borrow_mut().push(Invocation::Update);
            Trans::None
        }
    }

    fn get_window_event(events_loop: &mut EventsLoop) -> Event {
        let mut enigo = Enigo::new();
        enigo.key_click(Key::Backspace);

        let mut return_event = None;

        events_loop.run_forever(|event| {
            if match &event {
                &Event::WindowEvent {
                    event: ref window_event,
                    ..
                } => match window_event {
                    &WindowEvent::KeyboardInput { .. } => true,
                    _ => false,
                },
                _ => false,
            } {
                return_event = Some(event);
                ControlFlow::Break
            } else {
                ControlFlow::Continue
            }
        });

        events_loop.poll_events(|_event| {}); // empty event queue

        return_event.unwrap()
    }
}
