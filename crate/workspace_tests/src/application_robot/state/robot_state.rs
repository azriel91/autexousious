#[cfg(test)]
mod test {
    use std::{
        cell::RefCell,
        fmt::{self, Debug},
        rc::Rc,
    };

    use amethyst::{
        ecs::{World, WorldExt},
        State, StateData, Trans,
    };
    use debug_util_amethyst::assert_eq_trans;

    use application_robot::{Intercept, RobotState};

    type Invocations = Rc<RefCell<Vec<Invocation>>>;

    fn setup<T, E>(
        invocations: Invocations,
        intercepts: Vec<Rc<RefCell<dyn Intercept<T, E>>>>,
    ) -> (RobotState<T, E>, World, Invocations)
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        let robot_state = RobotState::new_with_intercepts(
            Box::new(MockState::new(invocations.clone(), Trans::None)),
            intercepts,
        );

        let world = World::new();

        (robot_state, world, invocations)
    }

    fn setup_without_intercepts<T, E>() -> (RobotState<T, E>, World, Invocations)
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        setup(Rc::new(RefCell::new(vec![])), Vec::new())
    }

    fn setup_with_no_op_intercepts<T, E>() -> (RobotState<T, E>, World, Invocations)
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        let invocations = Rc::new(RefCell::new(vec![]));
        let intercepts: Vec<Rc<RefCell<dyn Intercept<T, E>>>> = vec![
            Rc::new(RefCell::new(MockIntercept {
                id: 0,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: false,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 1,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: false,
            })),
        ];
        setup(invocations, intercepts)
    }

    fn setup_with_begin_intercepts<T, E>() -> (RobotState<T, E>, World, Invocations)
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        let invocations = Rc::new(RefCell::new(vec![]));
        let intercepts: Vec<Rc<RefCell<dyn Intercept<T, E>>>> = vec![
            Rc::new(RefCell::new(MockIntercept {
                id: 0,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: false,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 1,
                invocations: invocations.clone(),
                trans_begin: Some(Trans::Pop),
                trans_end: None,
                transitive: false,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 2,
                invocations: invocations.clone(),
                trans_begin: Some(Trans::Quit),
                trans_end: None,
                transitive: false,
            })),
        ];
        setup(invocations, intercepts)
    }

    fn setup_with_push_begin_intercepts<T, E>() -> (RobotState<T, E>, World, Invocations)
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        let invocations = Rc::new(RefCell::new(vec![]));
        let intercepts: Vec<Rc<RefCell<dyn Intercept<T, E>>>> = vec![
            Rc::new(RefCell::new(MockIntercept {
                id: 0,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: false,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 3,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: true,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 4,
                invocations: invocations.clone(),
                trans_begin: Some(Trans::Push(Box::new(MockState::new(
                    invocations.clone(),
                    Trans::None,
                )))),
                trans_end: None,
                transitive: false,
            })), // kcov-ignore
        ];
        setup(invocations, intercepts)
    }

    fn setup_with_push_end_intercepts<T, E>() -> (RobotState<T, E>, World, Invocations)
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        let invocations = Rc::new(RefCell::new(vec![]));
        let intercepts: Vec<Rc<RefCell<dyn Intercept<T, E>>>> = vec![
            Rc::new(RefCell::new(MockIntercept {
                id: 0,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: false,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 3,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: true,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 4,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: Some(Trans::Push(Box::new(MockState::new(
                    invocations.clone(),
                    Trans::None,
                )))),
                transitive: false,
            })), // kcov-ignore
        ];
        setup(invocations, intercepts)
    }

    fn setup_with_switch_begin_intercepts<T, E>() -> (RobotState<T, E>, World, Invocations)
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        let invocations = Rc::new(RefCell::new(vec![]));
        let intercepts: Vec<Rc<RefCell<dyn Intercept<T, E>>>> = vec![
            Rc::new(RefCell::new(MockIntercept {
                id: 0,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: false,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 3,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: true,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 5,
                invocations: invocations.clone(),
                trans_begin: Some(Trans::Switch(Box::new(MockState::new(
                    invocations.clone(),
                    Trans::None,
                )))),
                trans_end: None,
                transitive: false,
            })), // kcov-ignore
        ];
        setup(invocations, intercepts)
    }

    fn setup_with_switch_end_intercepts<T, E>() -> (RobotState<T, E>, World, Invocations)
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        let invocations = Rc::new(RefCell::new(vec![]));
        let intercepts: Vec<Rc<RefCell<dyn Intercept<T, E>>>> = vec![
            Rc::new(RefCell::new(MockIntercept {
                id: 0,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: false,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 3,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: true,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 5,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: Some(Trans::Switch(Box::new(MockState::new(
                    invocations.clone(),
                    Trans::None,
                )))),
                transitive: false,
            })), // kcov-ignore
        ];
        setup(invocations, intercepts)
    }

    fn setup_with_end_intercepts<T, E>() -> (RobotState<T, E>, World, Invocations)
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        let invocations = Rc::new(RefCell::new(vec![]));
        let intercepts: Vec<Rc<RefCell<dyn Intercept<T, E>>>> = vec![
            Rc::new(RefCell::new(MockIntercept {
                id: 0,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: None,
                transitive: false,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 1,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: Some(Trans::Pop),
                transitive: false,
            })),
            Rc::new(RefCell::new(MockIntercept {
                id: 2,
                invocations: invocations.clone(),
                trans_begin: None,
                trans_end: Some(Trans::Quit),
                transitive: false,
            })),
        ];
        setup(invocations, intercepts)
    }

    #[macro_use]
    macro_rules! delegate_test {
        ($test_name:ident, $function:ident, $invocation:expr) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
                    setup_without_intercepts();

                state.$function(StateData::new(&mut world, &mut ()));

                assert_eq!(vec![$invocation], *invocations.borrow());
            }
        };
    }

    #[macro_use]
    macro_rules! intercept_no_op_test {
        ($test_name:ident, $function:ident, $($invocation:expr),* $(,)*) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
                    setup_with_no_op_intercepts();

                state.$function(StateData::new(&mut world, &mut ()));

                assert_eq!(
                    vec![$($invocation,)*],
                    *invocations.borrow()
                );
            }
        }
    }

    #[macro_use]
    macro_rules! intercept_no_op_trans_test {
        ($test_name:ident, $function:ident, $expected_trans:expr, $($invocation:expr),* $(,)*) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
                setup_with_no_op_intercepts();

                let trans = state.$function(StateData::new(&mut world, &mut ()));

                assert_eq_trans(&$expected_trans, &trans);
                assert_eq!(
                    vec![$($invocation,)*],
                    *invocations.borrow()
                );
            }
        }
    }

    #[macro_use]
    macro_rules! intercept_begin_test {
        ($test_name:ident, $function:ident, $expected_trans:expr, $($invocation:expr),* $(,)*) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
                    setup_with_begin_intercepts();

                let trans = state.$function(StateData::new(&mut world, &mut ()));

                assert_eq_trans(&$expected_trans, &trans);
                assert_eq!(
                    vec![$($invocation,)*],
                    *invocations.borrow()
                );
            }
        }
    }

    #[macro_use]
    macro_rules! intercept_end_test {
        ($test_name:ident, $function:ident, $expected_trans:expr, $($invocation:expr),* $(,)*) => {
            #[test]
            fn $test_name() {
                let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
                    setup_with_end_intercepts();

                let trans = state.$function(StateData::new(&mut world, &mut ()));

                assert_eq_trans(&$expected_trans, &trans);
                assert_eq!(
                    vec![$($invocation,)*],
                    *invocations.borrow()
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
    fn handle_event_delegates_to_state() {
        let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
            setup_without_intercepts();

        let trans = state.handle_event(StateData::new(&mut world, &mut ()), ());

        assert_eq_trans(&Trans::None, &trans);
        assert_eq!(vec![Invocation::HandleEvent], *invocations.borrow());
    }

    #[test]
    fn handle_event_invokes_intercept() {
        let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
            setup_with_no_op_intercepts();

        let trans = state.handle_event(StateData::new(&mut world, &mut ()), ());

        assert_eq_trans(&Trans::None, &trans);
        assert_eq!(
            vec![
                Invocation::HandleEventBegin(0),
                Invocation::HandleEventBegin(1),
                Invocation::HandleEvent,
                Invocation::HandleEventEnd(0),
                Invocation::HandleEventEnd(1),
            ],
            *invocations.borrow()
        );
    }

    #[test]
    #[ignore]
    fn handle_event_returns_intercept_trans_begin() {
        let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
            setup_with_begin_intercepts();

        let trans = state.handle_event(StateData::new(&mut world, &mut ()), ());

        assert_eq_trans(&Trans::Pop, &trans);
        assert_eq!(
            vec![
                Invocation::HandleEventBegin(0),
                Invocation::HandleEventBegin(1),
            ],
            *invocations.borrow()
        );
    }

    #[test]
    #[ignore]
    fn handle_event_returns_intercept_trans_end() {
        let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
            setup_with_end_intercepts();

        let trans = state.handle_event(StateData::new(&mut world, &mut ()), ());

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
            *invocations.borrow()
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

    #[test]
    fn intercept_begin_push_state_is_wrapped_with_robot_state_with_transitive_intercepts() {
        let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
            setup_with_push_begin_intercepts();

        let mut trans = state.update(StateData::new(&mut world, &mut ()));

        let dummy_state = MockState::new(Rc::new(RefCell::new(vec![])), Trans::None);
        let expected_trans = Trans::Push(Box::new(dummy_state));
        assert_eq_trans(&expected_trans, &trans);

        if let Trans::Push(ref mut pushed_state) = trans {
            pushed_state.update(StateData::new(&mut world, &mut ()));
        }

        assert_eq!(
            vec![
                Invocation::UpdateBegin(0),
                Invocation::UpdateBegin(3),
                Invocation::UpdateBegin(4),
                // Push
                Invocation::UpdateBegin(3),
                Invocation::Update,
                Invocation::UpdateEnd(3),
            ],
            *invocations.borrow()
        );
    }

    #[test]
    fn intercept_end_push_state_is_wrapped_with_robot_state_with_transitive_intercepts() {
        let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
            setup_with_push_end_intercepts();

        let mut trans = state.update(StateData::new(&mut world, &mut ()));

        let dummy_state = MockState::new(Rc::new(RefCell::new(vec![])), Trans::None);
        let expected_trans = Trans::Push(Box::new(dummy_state));
        assert_eq_trans(&expected_trans, &trans);

        if let Trans::Push(ref mut pushed_state) = trans {
            pushed_state.update(StateData::new(&mut world, &mut ()));
        }

        assert_eq!(
            vec![
                Invocation::UpdateBegin(0),
                Invocation::UpdateBegin(3),
                Invocation::UpdateBegin(4),
                Invocation::Update,
                Invocation::UpdateEnd(0),
                Invocation::UpdateEnd(3),
                Invocation::UpdateEnd(4),
                // Push
                Invocation::UpdateBegin(3),
                Invocation::Update,
                Invocation::UpdateEnd(3),
            ],
            *invocations.borrow()
        );
    }

    #[test]
    fn intercept_begin_switch_state_is_wrapped_with_robot_state_with_transitive_intercepts() {
        let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
            setup_with_switch_begin_intercepts();

        let mut trans = state.update(StateData::new(&mut world, &mut ()));

        let dummy_state = MockState::new(Rc::new(RefCell::new(vec![])), Trans::None);
        let expected_trans = Trans::Switch(Box::new(dummy_state));
        assert_eq_trans(&expected_trans, &trans);

        if let Trans::Switch(ref mut pushed_state) = trans {
            pushed_state.update(StateData::new(&mut world, &mut ()));
        }

        assert_eq!(
            vec![
                Invocation::UpdateBegin(0),
                Invocation::UpdateBegin(3),
                Invocation::UpdateBegin(5),
                // Switch
                Invocation::UpdateBegin(3),
                Invocation::Update,
                Invocation::UpdateEnd(3),
            ],
            *invocations.borrow()
        );
    }

    #[test]
    fn intercept_end_switch_state_is_wrapped_with_robot_state_with_transitive_intercepts() {
        let (mut state, mut world, invocations): (RobotState<(), ()>, _, _) =
            setup_with_switch_end_intercepts();

        let mut trans = state.update(StateData::new(&mut world, &mut ()));

        let dummy_state = MockState::new(Rc::new(RefCell::new(vec![])), Trans::None);
        let expected_trans = Trans::Switch(Box::new(dummy_state));
        assert_eq_trans(&expected_trans, &trans);

        if let Trans::Switch(ref mut pushed_state) = trans {
            pushed_state.update(StateData::new(&mut world, &mut ()));
        }

        assert_eq!(
            vec![
                Invocation::UpdateBegin(0),
                Invocation::UpdateBegin(3),
                Invocation::UpdateBegin(5),
                Invocation::Update,
                Invocation::UpdateEnd(0),
                Invocation::UpdateEnd(3),
                Invocation::UpdateEnd(5),
                // Switch
                Invocation::UpdateBegin(3),
                Invocation::Update,
                Invocation::UpdateEnd(3),
            ],
            *invocations.borrow()
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
            fn $function(&mut self, _: StateData<'_, T>) {
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
        ($function:ident, $invocation:expr; [$($additional_param:ty),*]) => {
            fn $function(&mut self, $(_: $additional_param),*) {
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
            fn $function(&mut self, $(_: $additional_param),*) -> Trans<T, E> {
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
            fn $function(&mut self, $(_: $additional_param),*) -> Option<Trans<T, E>> {
                self.invocations
                    .borrow_mut()
                    .push($invocation(self.id));

                self.$trans.take()
            }
        }
    }

    struct MockIntercept<T, E>
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        id: u32,
        invocations: Invocations,
        trans_begin: Option<Trans<T, E>>,
        trans_end: Option<Trans<T, E>>,
        transitive: bool,
    }

    impl<T, E> Intercept<T, E> for MockIntercept<T, E>
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        fn_id!(on_start_begin, Invocation::OnStartBegin; [&mut StateData<'_, T>]);
        fn_id!(on_start_end, Invocation::OnStartEnd; []);
        fn_id!(on_stop_begin, Invocation::OnStopBegin; [&mut StateData<'_, T>]);
        fn_id!(on_stop_end, Invocation::OnStopEnd; []);
        fn_id!(on_pause_begin, Invocation::OnPauseBegin; [&mut StateData<'_, T>]);
        fn_id!(on_pause_end, Invocation::OnPauseEnd; []);
        fn_id!(on_resume_begin, Invocation::OnResumeBegin; [&mut StateData<'_, T>]);
        fn_id!(on_resume_end, Invocation::OnResumeEnd; []);
        fn_opt_trans!(
            handle_event_begin,
            Invocation::HandleEventBegin,
            trans_begin;
            [&mut StateData<'_, T>, &mut E]
        );
        fn_opt_trans!(handle_event_end, Invocation::HandleEventEnd, trans_end; [&Trans<T, E>]);
        fn_opt_trans!(fixed_update_begin, Invocation::FixedUpdateBegin, trans_begin; [&mut StateData<'_, T>]);
        fn_opt_trans!(fixed_update_end, Invocation::FixedUpdateEnd, trans_end; [&Trans<T, E>]);
        fn_opt_trans!(update_begin, Invocation::UpdateBegin, trans_begin; [&mut StateData<'_, T>]);
        fn_opt_trans!(update_end, Invocation::UpdateEnd, trans_end; [&Trans<T, E>]);
        fn is_transitive(&self) -> bool {
            self.transitive
        }
    }

    // kcov-ignore-start
    impl<T, E> Debug for MockIntercept<T, E>
    where
        E: Send + Sync + 'static,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            write!(
                f,
                "MockIntercept {{ invocations: {:?}, trans_begin: {:?}, trans_end: {:?} }}",
                self.invocations, &self.trans_begin, &self.trans_end,
            )
        }
    }
    // kcov-ignore-end

    #[derive(Default)]
    struct MockState<T, E>
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        invocations: Invocations,
        trans: Option<Trans<T, E>>,
    }

    impl<T, E> MockState<T, E>
    where
        E: Send + Sync + 'static,
    {
        fn new(invocations: Invocations, trans: Trans<T, E>) -> Self {
            MockState {
                invocations,
                trans: Some(trans),
            }
        }
    }

    impl<T, E> Debug for MockState<T, E>
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            write!(
                f,
                "MockState {{ invocations: {:?}, trans: {:?} }}",
                self.invocations, &self.trans,
            )
        }
    }

    impl<'a, 'b, T, E> State<T, E> for MockState<T, E>
    where
        T: 'static,
        E: Send + Sync + 'static,
    {
        fn_!(on_start, Invocation::OnStart);
        fn_!(on_stop, Invocation::OnStop);
        fn_!(on_pause, Invocation::OnPause);
        fn_!(on_resume, Invocation::OnResume);
        fn_trans!(
            handle_event,
            Invocation::HandleEvent; // kcov-ignore
            [StateData<'_, T>, E]
        );
        fn_trans!(fixed_update, Invocation::FixedUpdate; [StateData<'_, T>]);
        fn_trans!(update, Invocation::Update; [StateData<'_, T>]);
    }
}
