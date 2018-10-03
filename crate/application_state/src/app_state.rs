use std::fmt::Debug;

use amethyst::{core::SystemBundle, ecs::prelude::*, prelude::*};
use application_event::AppEvent;

use AutexState;

/// Wrapper `State` with a custom dispatcher.
///
/// All `State` methods are called through, with special handling for the following:
///
/// * `on_start`: The `World` is setup with the state specific dispatcher before calling through.
/// * `update`: The game data and state specific dispatchers are run before calling through.
///
/// This state is not intended to be constructed directly, but through the
/// [`AppStateBuilder`][state_builder].
///
/// # Type Parameters
///
/// * `S`: `State` to delegate to.
///
/// [state_builder]: application_state/struct.AppStateBuilder.html
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct AppState<'a, 'b, S>
where
    S: AutexState<'a, 'b>,
{
    /// State specific dispatcher.
    #[derivative(Debug = "ignore")]
    dispatcher: Dispatcher<'a, 'b>,
    /// The `State` to delegate to.
    #[derivative(Debug(bound = "S: Debug"))]
    delegate: S,
}

impl<'a, 'b, S> AppState<'a, 'b, S>
where
    S: AutexState<'a, 'b>,
{
    /// Sets up the dispatcher for this state.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to operate on.
    fn initialize_dispatcher(&mut self, world: &mut World) {
        self.dispatcher.setup(&mut world.res);
    }
}

impl<'a, 'b, S> State<GameData<'a, 'b>, AppEvent> for AppState<'a, 'b, S>
where
    S: AutexState<'a, 'b>,
{
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.initialize_dispatcher(&mut data.world);
        self.delegate.on_start(data);
    }

    fn on_stop(&mut self, data: StateData<GameData<'a, 'b>>) {
        self.delegate.on_stop(data);
    }

    fn on_pause(&mut self, data: StateData<GameData<'a, 'b>>) {
        self.delegate.on_pause(data);
    }

    fn on_resume(&mut self, data: StateData<GameData<'a, 'b>>) {
        self.delegate.on_resume(data);
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
        event: StateEvent<AppEvent>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        self.delegate.handle_event(data, event)
    }

    fn fixed_update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        self.delegate.fixed_update(data)
    }

    fn update(&mut self, data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>, AppEvent> {
        // Note: The built-in dispatcher must be run before the state specific dispatcher as the
        // `"input_system"` is registered in the main dispatcher, and by design we have chosen that
        // systems that depend on that should be placed in the state specific dispatcher.
        data.data.update(&data.world);
        self.dispatcher.dispatch(&data.world.res);

        self.delegate.update(data)
    }
}

/// Builder for an `AppState`.
///
/// `SystemBundle`s to run in the `AppState`'s dispatcher are registered on this builder.
///
/// # Type Parameters
///
/// * `S`: `State` to delegate to.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct AppStateBuilder<'a, 'b, S>
where
    S: AutexState<'a, 'b>,
{
    /// State specific dispatcher builder.
    #[derivative(Debug = "ignore")]
    #[new(value = "DispatcherBuilder::new()")]
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
    /// The `State` to delegate to.
    #[derivative(Debug(bound = "S: Debug"))]
    delegate: S,
}

impl<'a, 'b, S> AppStateBuilder<'a, 'b, S>
where
    S: AutexState<'a, 'b>,
{
    /// Registers a bundle whose systems to run in the `AppState`.
    ///
    /// # Parameters
    ///
    /// * `bundle`: Bundle to register.
    pub fn with_bundle<B: SystemBundle<'a, 'b>>(mut self, bundle: B) -> Self {
        bundle
            .build(&mut self.dispatcher_builder)
            .expect("Failed to register bundle for `AppState`.");
        self
    }

    /// Registers a system to run in the `AppState`.
    ///
    /// # Parameters
    ///
    /// * `system`: Bundle to register.
    pub fn with_system<Sys>(mut self, system: Sys, name: &str, deps: &[&str]) -> Self
    where
        Sys: for<'s> System<'s> + Send + Sync + 'a,
    {
        self.dispatcher_builder.add(system, name, deps);
        self
    }

    /// Builds and returns the `AppState`.
    pub fn build(self) -> AppState<'a, 'b, S> {
        AppState::new(self.dispatcher_builder.build(), self.delegate)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;

    use amethyst::{ecs::prelude::*, prelude::*};
    use application_event::AppEvent;
    use character_selection_model::CharacterSelectionEvent;
    use rayon::ThreadPoolBuilder;

    use super::{AppState, AppStateBuilder};

    type Invocations = Rc<RefCell<Vec<Invocation>>>;

    macro_rules! test_delegate {
        ($test_name:ident, $function:ident, $invocation:expr) => {
            #[test]
            fn $test_name() {
                let (mut world, mut game_data, invocations, mut state) = setup();

                state.$function(StateData::new(&mut world, &mut game_data));

                assert_eq!(vec![$invocation], *invocations.borrow());
            }
        };
    }

    test_delegate!(delegates_on_start, on_start, Invocation::OnStart);
    test_delegate!(delegates_on_stop, on_stop, Invocation::OnStop);
    test_delegate!(delegates_on_pause, on_pause, Invocation::OnPause);
    test_delegate!(delegates_on_resume, on_resume, Invocation::OnResume);
    test_delegate!(
        delegates_fixed_update,
        fixed_update,
        Invocation::FixedUpdate
    );
    test_delegate!(delegates_update, update, Invocation::Update);

    #[test]
    fn delegates_handle_event() {
        let (mut world, mut game_data, invocations, mut state) = setup();

        let event = StateEvent::Custom(AppEvent::CharacterSelection(
            CharacterSelectionEvent::Confirm,
        ));

        state.handle_event(StateData::new(&mut world, &mut game_data), event);

        assert_eq!(vec![Invocation::HandleEvent], *invocations.borrow());
    }

    #[test]
    fn on_start_sets_up_world_for_state_specific_dispatcher() {
        let game_data_builder = GameDataBuilder::default();
        let (mut world, mut game_data, _invocations, mut state) =
            setup_with_system(game_data_builder, Some((SystemCounter, "", &[])));

        state.on_start(StateData::new(&mut world, &mut game_data));

        assert!(world.res.try_fetch::<Counter>().is_some());
    }

    #[test]
    fn update_runs_game_data_dispatcher_then_state_specific_dispatcher() {
        let game_data_builder = GameDataBuilder::default().with(SystemCounter, "", &[]);
        let (mut world, mut game_data, _invocations, mut state) =
            setup_with_system(game_data_builder, Some((SystemCopyCounter, "", &[])));

        state.on_start(StateData::new(&mut world, &mut game_data));
        state.update(StateData::new(&mut world, &mut game_data));

        let copy_counter = world.res.try_fetch::<CopyCounter>();
        assert!(copy_counter.is_some());
        assert_eq!(CopyCounter(10), *copy_counter.unwrap());
    }

    fn setup<'a, 'b>() -> (
        World,
        GameData<'a, 'b>,
        Invocations,
        AppState<'a, 'b, MockState>,
    ) {
        setup_with_system(
            GameDataBuilder::default(),
            None as Option<(SystemCounter, &str, &[&str])>,
        )
    }

    fn setup_with_system<'a, 'b, Sys>(
        game_data_builder: GameDataBuilder<'a, 'b>,
        system: Option<(Sys, &str, &[&str])>,
    ) -> (
        World,
        GameData<'a, 'b>,
        Invocations,
        AppState<'a, 'b, MockState>,
    )
    where
        Sys: for<'s> System<'s> + Send + Sync + 'a,
    {
        let mut world = World::new();
        world.add_resource(Arc::new(
            ThreadPoolBuilder::default()
                .build()
                .unwrap_or_else(|e| panic!("Failed to build ThreadPool. {}", e)),
        ));
        let game_data = game_data_builder.build(&mut world);
        let invocations = Rc::new(RefCell::new(vec![]));
        let state = {
            let mock_state = MockState {
                invocations: invocations.clone(),
            };
            let mut builder = AppStateBuilder::new(mock_state);
            if let Some((system, name, deps)) = system {
                builder = builder.with_system(system, name, deps)
            }

            builder.build()
        };

        (world, game_data, invocations, state)
    }

    #[derive(Default)]
    struct MockState {
        invocations: Invocations,
    }

    impl<'a, 'b> State<GameData<'a, 'b>, AppEvent> for MockState {
        fn on_start(&mut self, mut _data: StateData<GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnStart);
        }
        fn on_stop(&mut self, _data: StateData<GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnStop);
        }
        fn on_pause(&mut self, _data: StateData<GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnPause);
        }

        fn on_resume(&mut self, _data: StateData<GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnResume);
        }

        fn handle_event(
            &mut self,
            _data: StateData<GameData<'a, 'b>>,
            _event: StateEvent<AppEvent>,
        ) -> Trans<GameData<'a, 'b>, AppEvent> {
            self.invocations.borrow_mut().push(Invocation::HandleEvent);
            Trans::None
        }

        fn fixed_update(
            &mut self,
            _data: StateData<GameData<'a, 'b>>,
        ) -> Trans<GameData<'a, 'b>, AppEvent> {
            self.invocations.borrow_mut().push(Invocation::FixedUpdate);
            Trans::None
        }

        fn update(
            &mut self,
            _data: StateData<GameData<'a, 'b>>,
        ) -> Trans<GameData<'a, 'b>, AppEvent> {
            self.invocations.borrow_mut().push(Invocation::Update);
            Trans::None
        }
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

    #[derive(Debug, PartialEq)]
    struct Counter(u32);
    #[derive(Debug, Default, PartialEq)]
    struct CopyCounter(u32);

    #[derive(Debug)]
    struct SystemCounter;
    impl<'s> System<'s> for SystemCounter {
        type SystemData = WriteExpect<'s, Counter>;
        fn run(&mut self, mut counter: Self::SystemData) {
            *counter = Counter((*counter).0 + 10);
        }

        fn setup(&mut self, res: &mut Resources) {
            Self::SystemData::setup(res);
            res.insert(Counter(0));
        }
    }

    #[derive(Debug)]
    struct SystemCopyCounter;
    impl<'s> System<'s> for SystemCopyCounter {
        type SystemData = (ReadExpect<'s, Counter>, Write<'s, CopyCounter>);
        fn run(&mut self, (counter, mut copy_counter): Self::SystemData) {
            *copy_counter = CopyCounter((*counter).0);
        }
    }
}
