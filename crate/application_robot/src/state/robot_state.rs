use std::fmt::Debug;

use amethyst::prelude::*;
use amethyst::renderer::Event;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use state::Intercept;
use state::intercept::ApplicationEventIntercept;

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
    pub intercepts: Vec<Box<Intercept>>,
    /// State to delegate behaviour to.
    pub delegate: Box<S>,
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

    fn fold_trans_begin<F>(&mut self, mut intercept_fn: F) -> Option<Trans>
    where
        F: FnMut(&mut Box<Intercept>) -> Option<Trans>,
    {
        self.intercepts
            .iter_mut()
            .fold_while(None, |trans, intercept| {
                if trans.is_none() {
                    Continue(intercept_fn(intercept))
                } else {
                    Done(trans)
                }
            })
            .into_inner()
    }

    fn fold_trans_end<F>(&mut self, state_trans: Trans, mut intercept_fn: F) -> Trans
    where
        F: FnMut(&mut Box<Intercept>, &Trans) -> Option<Trans>,
    {
        let intercept_trans = {
            let state_trans_ref = &state_trans;
            self.intercepts
                .iter_mut()
                .fold_while(None, |trans, intercept| {
                    if trans.is_none() {
                        Continue(intercept_fn(intercept, state_trans_ref))
                    } else {
                        Done(trans)
                    }
                })
                .into_inner()
        };
        intercept_trans.unwrap_or(state_trans)
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

    // TODO: Pending <https://gitlab.com/azriel91/autexousious/issues/16>
    // kcov-ignore-start
    fn handle_event(&mut self, world: &mut World, mut event: Event) -> Trans {
        let intercept_trans =
            self.fold_trans_begin(|intercept| intercept.handle_event_begin(world, &mut event));
        if let Some(trans) = intercept_trans {
            return trans;
        }

        let trans = self.delegate.handle_event(world, event);

        self.fold_trans_end(trans, |intercept, trans| {
            intercept.handle_event_end(world, trans)
        })
    }
    // kcov-ignore-end

    fn fixed_update(&mut self, world: &mut World) -> Trans {
        let intercept_trans =
            self.fold_trans_begin(|intercept| intercept.fixed_update_begin(world));
        if let Some(trans) = intercept_trans {
            return trans;
        }

        let trans = self.delegate.fixed_update(world);

        self.fold_trans_end(trans, |intercept, trans| {
            intercept.fixed_update_end(world, trans)
        })
    }

    fn update(&mut self, world: &mut World) -> Trans {
        let intercept_trans = self.fold_trans_begin(|intercept| intercept.update_begin(world));
        if let Some(trans) = intercept_trans {
            return trans;
        }

        let trans = self.delegate.update(world);

        self.fold_trans_end(trans, |intercept, trans| intercept.update_end(world, trans))
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::fmt::{self, Debug};
    use std::rc::Rc;

    use amethyst::ecs::World;
    use amethyst::prelude::*;
    use amethyst::renderer::{Event, WindowEvent};
    use debug_util_amethyst::{assert_eq_trans, display_trans};
    use enigo::{Enigo, Key, KeyboardControllable};
    use winit::{ControlFlow, EventsLoop, Window};

    use super::{RobotState, RobotStateBuilder};
    use state::Intercept;

    fn setup(
        invocations: Rc<RefCell<Vec<Invocation>>>,
        intercepts: Vec<Box<Intercept>>,
    ) -> (RobotState<MockState>, World) {
        let mut robot_state = RobotStateBuilder::default()
                .delegate(Box::new(MockState::new(invocations.clone(), Some(Trans::None))))
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

    fn setup_with_no_op_intercepts() -> (RobotState<MockState>, World) {
        let invocations = Rc::new(RefCell::new(vec![]));
        let intercepts: Vec<Box<Intercept>> = vec![
            Box::new(MockIntercept {
                id: 0,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
            }),
            Box::new(MockIntercept {
                id: 1,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
            }),
        ];
        setup(invocations, intercepts)
    }

    fn setup_with_begin_intercepts() -> (RobotState<MockState>, World) {
        let invocations = Rc::new(RefCell::new(vec![]));
        let intercepts: Vec<Box<Intercept>> = vec![
            Box::new(MockIntercept {
                id: 0,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
            }),
            Box::new(MockIntercept {
                id: 1,
                invocations: invocations.clone(),
                trans_begin: Some(Trans::Pop),
                trans_end: None,
            }),
            Box::new(MockIntercept {
                id: 2,
                invocations: invocations.clone(),
                trans_begin: Some(Trans::Quit),
                trans_end: None,
            }),
        ];
        setup(invocations, intercepts)
    }

    fn setup_with_end_intercepts() -> (RobotState<MockState>, World) {
        let invocations = Rc::new(RefCell::new(vec![]));
        let intercepts: Vec<Box<Intercept>> = vec![
            Box::new(MockIntercept {
                id: 0,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
            }),
            Box::new(MockIntercept {
                id: 1,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: Some(Trans::Pop),
            }),
            Box::new(MockIntercept {
                id: 2,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: Some(Trans::Quit),
            }),
        ];
        setup(invocations, intercepts)
    }

    #[macro_use]
    macro_rules! delegate_test {
        ($test_name:ident, $function:ident, $invocation:expr) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world) = setup_without_intercepts();

                state.$function(&mut world);

                assert_eq!(vec![$invocation], *state.delegate.invocations.borrow());
            }
        };
    }

    #[macro_use]
    macro_rules! intercept_no_op_test {
        ($test_name:ident, $function:ident, $($invocation:expr),* $(,)*) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world) = setup_with_no_op_intercepts();

                state.$function(&mut world);

                assert_eq!(
                    vec![$($invocation,)*],
                    *state.delegate.invocations.borrow()
                );
            }
        }
    }

    #[macro_use]
    macro_rules! intercept_no_op_trans_test {
        ($test_name:ident, $function:ident, $expected_trans:expr, $($invocation:expr),* $(,)*) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world) = setup_with_no_op_intercepts();

                let trans = state.$function(&mut world);

                assert_eq_trans(&$expected_trans, &trans);
                assert_eq!(
                    vec![$($invocation,)*],
                    *state.delegate.invocations.borrow()
                );
            }
        }
    }

    #[macro_use]
    macro_rules! intercept_begin_test {
        ($test_name:ident, $function:ident, $expected_trans:expr, $($invocation:expr),* $(,)*) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world) = setup_with_begin_intercepts();

                let trans = state.$function(&mut world);

                assert_eq_trans(&$expected_trans, &trans);
                assert_eq!(
                    vec![$($invocation,)*],
                    *state.delegate.invocations.borrow()
                );
            }
        }
    }

    #[macro_use]
    macro_rules! intercept_end_test {
        ($test_name:ident, $function:ident, $expected_trans:expr, $($invocation:expr),* $(,)*) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world) = setup_with_end_intercepts();

                let trans = state.$function(&mut world);

                assert_eq_trans(&$expected_trans, &trans);
                assert_eq!(
                    vec![$($invocation,)*],
                    *state.delegate.invocations.borrow()
                );
            }
        }
    }

    delegate_test!(on_start_delegates_to_state, on_start, Invocation::OnStart);
    delegate_test!(on_stop_delegates_to_state, on_stop, Invocation::OnStop);
    delegate_test!(on_pause_delegates_to_state, on_pause, Invocation::OnPause);
    delegate_test!(
        on_resume_delegates_to_state,
        on_resume,
        Invocation::OnResume
    );
    delegate_test!(
        fixed_update_delegates_to_state,
        fixed_update,
        Invocation::FixedUpdate
    );
    delegate_test!(update_delegates_to_state, update, Invocation::Update);

    // TODO: We ignore running the following tests because we cannot construct a window in both
    // this test and in the application_event_intercept module due to
    // <https://gitlab.com/azriel91/autexousious/issues/16>.
    // kcov-ignore-start
    #[test]
    #[ignore]
    fn handle_event_delegates_to_state() {
        let (mut state, mut world) = setup_without_intercepts();

        let mut events_loop = EventsLoop::new();
        let _window = Window::new(&events_loop).unwrap();
        let event = get_window_event(&mut events_loop);

        let trans = state.handle_event(&mut world, event);

        assert_eq_trans(&Trans::None, &trans);
        assert_eq!(
            vec![Invocation::HandleEvent],
            *state.delegate.invocations.borrow()
        );
    }

    #[test]
    #[ignore]
    fn handle_event_invokes_intercept() {
        let (mut state, mut world) = setup_with_no_op_intercepts();

        let mut events_loop = EventsLoop::new();
        let _window = Window::new(&events_loop).unwrap();
        let event = get_window_event(&mut events_loop);

        let trans = state.handle_event(&mut world, event);

        assert_eq_trans(&Trans::None, &trans);
        assert_eq!(
            vec![
                Invocation::HandleEventBegin(0),
                Invocation::HandleEventBegin(1),
                Invocation::HandleEvent,
                Invocation::HandleEventEnd(0),
                Invocation::HandleEventEnd(1),
            ],
            *state.delegate.invocations.borrow()
        );
    }

    #[test]
    #[ignore]
    fn handle_event_returns_intercept_trans_begin() {
        let (mut state, mut world) = setup_with_begin_intercepts();

        let mut events_loop = EventsLoop::new();
        let _window = Window::new(&events_loop).unwrap();
        let event = get_window_event(&mut events_loop);

        let trans = state.handle_event(&mut world, event);

        assert_eq_trans(&Trans::Pop, &trans);
        assert_eq!(
            vec![
                Invocation::HandleEventBegin(0),
                Invocation::HandleEventBegin(1),
            ],
            *state.delegate.invocations.borrow()
        );
    }

    #[test]
    #[ignore]
    fn handle_event_returns_intercept_trans_end() {
        let (mut state, mut world) = setup_with_end_intercepts();

        let mut events_loop = EventsLoop::new();
        let _window = Window::new(&events_loop).unwrap();
        let event = get_window_event(&mut events_loop);

        let trans = state.handle_event(&mut world, event);

        assert_eq_trans(&Trans::Pop, &trans);
        assert_eq!(
            vec![
                Invocation::HandleEventBegin(0),
                Invocation::HandleEventBegin(1),
                Invocation::HandleEventBegin(2),
                Invocation::HandleEvent,
                Invocation::HandleEventEnd(0),
                Invocation::HandleEventEnd(1),
            ],
            *state.delegate.invocations.borrow()
        );
    }
    // kcov-ignore-end

    intercept_no_op_test!(
        on_start_invokes_intercept,
        on_start,
        Invocation::OnStartBegin(0),
        Invocation::OnStartBegin(1),
        Invocation::OnStart,
        Invocation::OnStartEnd(0),
        Invocation::OnStartEnd(1),
    );
    intercept_no_op_test!(
        on_stop_invokes_intercept,
        on_stop,
        Invocation::OnStopBegin(0),
        Invocation::OnStopBegin(1),
        Invocation::OnStop,
        Invocation::OnStopEnd(0),
        Invocation::OnStopEnd(1),
    );
    intercept_no_op_test!(
        on_pause_invokes_intercept,
        on_pause,
        Invocation::OnPauseBegin(0),
        Invocation::OnPauseBegin(1),
        Invocation::OnPause,
        Invocation::OnPauseEnd(0),
        Invocation::OnPauseEnd(1),
    );
    intercept_no_op_test!(
        on_resume_invokes_intercept,
        on_resume,
        Invocation::OnResumeBegin(0),
        Invocation::OnResumeBegin(1),
        Invocation::OnResume,
        Invocation::OnResumeEnd(0),
        Invocation::OnResumeEnd(1),
    );
    intercept_no_op_trans_test!(
        fixed_update_invokes_intercept,
        fixed_update,
        Trans::None,
        Invocation::FixedUpdateBegin(0),
        Invocation::FixedUpdateBegin(1),
        Invocation::FixedUpdate,
        Invocation::FixedUpdateEnd(0),
        Invocation::FixedUpdateEnd(1),
    );
    intercept_no_op_trans_test!(
        update_invokes_intercept,
        update,
        Trans::None,
        Invocation::UpdateBegin(0),
        Invocation::UpdateBegin(1),
        Invocation::Update,
        Invocation::UpdateEnd(0),
        Invocation::UpdateEnd(1),
    );

    intercept_begin_test!(
        fixed_update_returns_intercept_trans_begin,
        fixed_update,
        Trans::Pop,
        Invocation::FixedUpdateBegin(0),
        Invocation::FixedUpdateBegin(1),
    );

    intercept_begin_test!(
        update_returns_intercept_trans_begin,
        update,
        Trans::Pop,
        Invocation::UpdateBegin(0),
        Invocation::UpdateBegin(1),
    );

    intercept_end_test!(
        fixed_update_returns_intercept_trans_end,
        fixed_update,
        Trans::Pop,
        Invocation::FixedUpdateBegin(0),
        Invocation::FixedUpdateBegin(1),
        Invocation::FixedUpdateBegin(2),
        Invocation::FixedUpdate,
        Invocation::FixedUpdateEnd(0),
        Invocation::FixedUpdateEnd(1),
    );

    intercept_end_test!(
        update_returns_intercept_trans_end,
        update,
        Trans::Pop,
        Invocation::UpdateBegin(0),
        Invocation::UpdateBegin(1),
        Invocation::UpdateBegin(2),
        Invocation::Update,
        Invocation::UpdateEnd(0),
        Invocation::UpdateEnd(1),
    );

    #[derive(Debug, PartialEq)]
    enum Invocation {
        OnStart,
        OnStop,
        OnPause,
        OnResume,
        HandleEvent,
        FixedUpdate,
        Update,

        // `Intercept` invocations
        OnStartBegin(u32),
        OnStartEnd(u32),
        OnStopBegin(u32),
        OnStopEnd(u32),
        OnPauseBegin(u32),
        OnPauseEnd(u32),
        OnResumeBegin(u32),
        OnResumeEnd(u32),
        HandleEventBegin(u32), // kcov-ignore
        HandleEventEnd(u32),   // kcov-ignore
        FixedUpdateBegin(u32),
        FixedUpdateEnd(u32),
        UpdateBegin(u32),
        UpdateEnd(u32),
    }

    /// Declares a function that pushes the specified invocation to the `self.invocations` field.
    #[macro_use]
    macro_rules! fn_ {
        ($function:ident, $invocation:expr) => {
            fn $function(&mut self, _: &mut World) {
                self.invocations
                    .borrow_mut()
                    .push($invocation); // kcov-ignore
            }
        }
    }

    /// Declares a function that pushes the specified invocation to the `self.invocations` field.
    ///
    /// This macro passes the `self.id` field as a parameter to the `Invocation` variant.
    #[macro_use]
    macro_rules! fn_id {
        ($function:ident, $invocation:expr) => {
            fn $function(&mut self, _: &mut World) {
                self.invocations
                    .borrow_mut()
                    .push($invocation(self.id));
            }
        }
    }

    /// Declares a function that pushes the specified invocation to the `self.invocations` field.
    ///
    /// The function returns the value in the `self.trans` field, which is expected to contain a
    /// value.
    #[macro_use]
    macro_rules! fn_trans {
        ($function:ident, $invocation:expr; [$($additional_param:ty),*]) => {
            fn $function(&mut self, _: &mut World, $(_: $additional_param),*) -> Trans {
                self.invocations
                    .borrow_mut()
                    .push($invocation); // kcov-ignore

                self.trans.take().unwrap()
            }
        }
    }

    /// Declares a function that pushes the specified invocation to the `self.invocations` field.
    ///
    /// The function returns the optional value in the `self.$trans` field
    #[macro_use]
    macro_rules! fn_opt_trans {
        ($function:ident, $invocation:expr, $trans:ident; [$($additional_param:ty),*]) => {
            fn $function(&mut self, _: &mut World, $(_: $additional_param),*) -> Option<Trans> {
                self.invocations
                    .borrow_mut()
                    .push($invocation(self.id));

                self.$trans.take()
            }
        }
    }

    struct MockIntercept {
        id: u32,
        invocations: Rc<RefCell<Vec<Invocation>>>,
        trans_begin: Option<Trans>,
        trans_end: Option<Trans>,
    }

    impl Intercept for MockIntercept {
        fn_id!(on_start_begin, Invocation::OnStartBegin);
        fn_id!(on_start_end, Invocation::OnStartEnd);
        fn_id!(on_stop_begin, Invocation::OnStopBegin);
        fn_id!(on_stop_end, Invocation::OnStopEnd);
        fn_id!(on_pause_begin, Invocation::OnPauseBegin);
        fn_id!(on_pause_end, Invocation::OnPauseEnd);
        fn_id!(on_resume_begin, Invocation::OnResumeBegin);
        fn_id!(on_resume_end, Invocation::OnResumeEnd);
        fn_opt_trans!(handle_event_begin, Invocation::HandleEventBegin, trans_begin; [&mut Event]);
        fn_opt_trans!(handle_event_end, Invocation::HandleEventEnd, trans_end; [&Trans]);
        fn_opt_trans!(fixed_update_begin, Invocation::FixedUpdateBegin, trans_begin; []);
        fn_opt_trans!(fixed_update_end, Invocation::FixedUpdateEnd, trans_end; [&Trans]);
        fn_opt_trans!(update_begin, Invocation::UpdateBegin, trans_begin; []);
        fn_opt_trans!(update_end, Invocation::UpdateEnd, trans_end; [&Trans]);
    }

    // kcov-ignore-start
    impl Debug for MockIntercept {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(
                f,
                "MockIntercept {{ invocations: {:?}, trans_begin: {}, trans_end: {} }}",
                self.invocations,
                format_trans(&self.trans_begin),
                format_trans(&self.trans_end),
            )
        }
    }
    // kcov-ignore-end

    #[derive(Default)]
    struct MockState {
        invocations: Rc<RefCell<Vec<Invocation>>>,
        trans: Option<Trans>,
    }

    impl MockState {
        fn new(invocations: Rc<RefCell<Vec<Invocation>>>, trans: Option<Trans>) -> Self {
            MockState { invocations, trans }
        }
    }

    impl Debug for MockState {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(
                f,
                "MockState {{ invocations: {:?}, trans: {} }}",
                self.invocations,
                format_trans(&self.trans),
            )
        }
    }

    impl State for MockState {
        fn_!(on_start, Invocation::OnStart);
        fn_!(on_stop, Invocation::OnStop);
        fn_!(on_pause, Invocation::OnPause);
        fn_!(on_resume, Invocation::OnResume);
        fn_trans!(handle_event, Invocation::HandleEvent; [Event]); // kcov-ignore
        fn_trans!(fixed_update, Invocation::FixedUpdate; []);
        fn_trans!(update, Invocation::Update; []);
    }

    // TODO: Pending <https://gitlab.com/azriel91/autexousious/issues/16>
    // kcov-ignore-start
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

    fn format_trans(trans: &Option<Trans>) -> String {
        if trans.is_some() {
            format!("Some({})", display_trans(trans.as_ref().unwrap()))
        } else {
            "None".to_string()
        }
    }
    // kcov-ignore-end
}
