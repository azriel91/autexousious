use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use amethyst::{
    core::SystemBundle,
    ecs::{Dispatcher, DispatcherBuilder, System, World},
    utils::removal::{self, Removal},
    GameData, State, StateData, Trans,
};
use application_event::AppEvent;
use derivative::Derivative;
use derive_new::new;
use log::debug;

use crate::{AutexState, HookFn, HookableFn};

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
pub struct AppState<'a, 'b, S, I>
where
    S: AutexState<'a, 'b>,
    I: Clone + Debug + Default + PartialEq + Send + Sync + 'static,
{
    /// State specific dispatcher.
    #[derivative(Debug = "ignore")]
    dispatcher: Dispatcher<'a, 'b>,
    /// The `State` to delegate to.
    #[derivative(Debug(bound = "S: Debug"))]
    delegate: S,
    /// Functions to run at the beginning of the various `State` methods.
    hook_fns: HashMap<HookableFn, Vec<HookFn>>,
    /// `PhantomData` for `Removal`.
    marker: PhantomData<I>,
}

impl<'a, 'b, S, I> AppState<'a, 'b, S, I>
where
    S: AutexState<'a, 'b>,
    I: Clone + Debug + Default + PartialEq + Send + Sync + 'static,
{
    /// Sets up the dispatcher for this state.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to operate on.
    fn initialize_dispatcher(&mut self, world: &mut World) {
        self.dispatcher.setup(&mut world.res);
    }

    fn remove_entities(world: &mut World) {
        removal::exec_removal(
            &*world.entities(),
            &world.read_storage::<Removal<I>>(),
            I::default(),
        ); // kcov-ignore
    }
}

impl<'a, 'b, S, I> State<GameData<'a, 'b>, AppEvent> for AppState<'a, 'b, S, I>
where
    S: AutexState<'a, 'b>,
    I: Clone + Debug + Default + PartialEq + Send + Sync + 'static,
{
    fn on_start(&mut self, mut data: StateData<'_, GameData<'a, 'b>>) {
        // Register the `Removal<I>` component first, because hook functions may rely on it.
        data.world.register::<Removal<I>>();

        if let Some(ref functions) = self.hook_fns.get(&HookableFn::OnStart) {
            functions
                .iter()
                .for_each(|function| function(&mut data.world));
        }

        self.initialize_dispatcher(&mut data.world);
        self.delegate.on_start(data);
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'a, 'b>>) {
        if let Some(ref functions) = self.hook_fns.get(&HookableFn::OnStop) {
            functions
                .iter()
                .for_each(|function| function(&mut data.world));
        }

        Self::remove_entities(&mut data.world);

        self.delegate.on_stop(data);
    }

    fn on_pause(&mut self, mut data: StateData<'_, GameData<'a, 'b>>) {
        if let Some(ref functions) = self.hook_fns.get(&HookableFn::OnPause) {
            functions
                .iter()
                .for_each(|function| function(&mut data.world));
        }

        Self::remove_entities(&mut data.world);

        self.delegate.on_pause(data);
    }

    fn on_resume(&mut self, mut data: StateData<'_, GameData<'a, 'b>>) {
        if let Some(ref functions) = self.hook_fns.get(&HookableFn::OnResume) {
            functions
                .iter()
                .for_each(|function| function(&mut data.world));
        }

        // Hacky way to reinitialize event channel `ReaderId`s.
        self.initialize_dispatcher(&mut data.world);
        self.delegate.on_resume(data);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
        event: AppEvent,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        debug!("Received event: {:?}", event);
        self.delegate.handle_event(data, event)
    }

    fn fixed_update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        self.delegate.fixed_update(data)
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
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
pub struct AppStateBuilder<'a, 'b, S, I>
where
    S: AutexState<'a, 'b>,
    I: Clone + Debug + Default + PartialEq + Send + Sync + 'static,
{
    /// State specific dispatcher builder.
    #[derivative(Debug = "ignore")]
    #[new(value = "DispatcherBuilder::new()")]
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
    /// The `State` to delegate to.
    #[derivative(Debug(bound = "S: Debug"))]
    delegate: S,
    /// Functions to run at the beginning of the various `State` methods.
    #[new(default)]
    hook_fns: HashMap<HookableFn, Vec<HookFn>>,
    /// `PhantomData` for `Removal`.
    marker: PhantomData<I>,
}

impl<'a, 'b, S, I> AppStateBuilder<'a, 'b, S, I>
where
    S: AutexState<'a, 'b>,
    I: Clone + Debug + Default + PartialEq + Send + Sync + 'static,
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

    /// Registers a function to be run at the beginning of a particular `State` method.
    ///
    /// # Parameters
    ///
    /// * `hookable_fn`: Method to run the function in.
    /// * `function`: Function to run.
    pub fn with_hook_fn(mut self, hookable_fn: HookableFn, function: HookFn) -> Self {
        self.hook_fns
            .entry(hookable_fn)
            .or_insert_with(Vec::new)
            .push(function);

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
    pub fn build(self) -> AppState<'a, 'b, S, I> {
        AppState::new(
            self.dispatcher_builder.build(),
            self.delegate,
            self.hook_fns,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc, sync::Arc};

    use amethyst::{
        ecs::{Builder, ReadExpect, System, World, Write, WriteExpect},
        utils::removal::Removal,
        DataInit, GameData, GameDataBuilder, State, StateData, Trans,
    };
    use application_event::AppEvent;
    use character_selection_model::CharacterSelectionEvent;
    use rayon::ThreadPoolBuilder;

    use super::{AppState, AppStateBuilder};
    use crate::{HookFn, HookableFn};

    type Invocations = Rc<RefCell<Vec<Invocation>>>;

    // === Delegation === //

    macro_rules! test_delegate {
        ($test_name:ident, $function:ident, $invocation:expr) => {
            #[test]
            fn $test_name() {
                let (mut world, mut game_data, invocations, mut state) = setup_with_defaults();

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
        let (mut world, mut game_data, invocations, mut state) = setup_with_defaults();

        let event = AppEvent::CharacterSelection(CharacterSelectionEvent::Confirm);

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

    // === `Removal` component === //

    #[test]
    fn on_start_registers_removal_component() {
        let (mut world, mut game_data, _invocations, mut state) = setup_without_removal();

        state.on_start(StateData::new(&mut world, &mut game_data));

        world.read_storage::<Removal<()>>(); // panics if it is not registered.
    }

    macro_rules! test_delete_removal_entities {
        ($test_name:ident, $method_name:ident) => {
            #[test]
            fn $test_name() {
                let (mut world, mut game_data, _invocations, mut state) = setup_with_defaults();
                let entity_with_removal = world.create_entity().with(Removal::new(())).build();
                let entity_without_removal = world.create_entity().build();

                state.$method_name(StateData::new(&mut world, &mut game_data));
                world.maintain();

                assert!(!world.is_alive(entity_with_removal));
                assert!(world.is_alive(entity_without_removal));
            }
        };
    }

    test_delete_removal_entities!(on_stop_deletes_entities_with_removal_component, on_stop);
    test_delete_removal_entities!(on_pause_deletes_entities_with_removal_component, on_pause);

    // === Dispatcher === //

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

    // === Hook functions === //

    macro_rules! test_hook_function {
        ($test_name:ident, $method_name:ident, $hook_fn_value:expr) => {
            #[test]
            fn $test_name() {
                let (mut world, mut game_data, _invocations, mut state) =
                    setup_with_hook_functions();

                state.$method_name(StateData::new(&mut world, &mut game_data));

                let hook_fn_value = world.res.try_fetch::<HookFnValue>();
                assert!(hook_fn_value.is_some());
                assert_eq!(HookFnValue($hook_fn_value), *hook_fn_value.unwrap());
            }
        };
    }

    test_hook_function!(on_start_runs_hook_functions, on_start, 1);
    test_hook_function!(on_stop_runs_hook_functions, on_stop, 2);
    test_hook_function!(on_pause_runs_hook_functions, on_pause, 4);
    test_hook_function!(on_resume_runs_hook_functions, on_resume, 8);

    // --- fixtures --- //

    fn setup_with_defaults<'a, 'b>() -> (
        World,
        GameData<'a, 'b>,
        Invocations,
        AppState<'a, 'b, MockState, ()>,
    ) {
        setup(
            true,
            false,
            GameDataBuilder::default(),
            None as Option<(SystemCounter, &str, &[&str])>,
        )
    }

    fn setup_without_removal<'a, 'b>() -> (
        World,
        GameData<'a, 'b>,
        Invocations,
        AppState<'a, 'b, MockState, ()>,
    ) {
        setup(
            false,
            false,
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
        AppState<'a, 'b, MockState, ()>,
    )
    where
        Sys: for<'s> System<'s> + Send + Sync + 'a,
    {
        setup(true, false, game_data_builder, system)
    }

    fn setup_with_hook_functions<'a, 'b>() -> (
        World,
        GameData<'a, 'b>,
        Invocations,
        AppState<'a, 'b, MockState, ()>,
    ) {
        setup(
            true,
            true,
            GameDataBuilder::default(),
            None as Option<(SystemCounter, &str, &[&str])>,
        )
    }

    fn setup<'a, 'b, Sys>(
        with_removal: bool,
        with_hook_fns: bool,
        game_data_builder: GameDataBuilder<'a, 'b>,
        system: Option<(Sys, &str, &[&str])>,
    ) -> (
        World,
        GameData<'a, 'b>,
        Invocations,
        AppState<'a, 'b, MockState, ()>,
    )
    where
        Sys: for<'s> System<'s> + Send + Sync + 'a,
    {
        let mut world = World::new();
        if with_removal {
            world.register::<Removal<()>>();
        }

        world.insert(Arc::new(
            ThreadPoolBuilder::default()
                .build()
                .unwrap_or_else(|e| panic!("Failed to build ThreadPool. {}", e)), // kcov-ignore
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

            if with_hook_fns {
                builder = builder
                    .with_hook_fn(
                        HookableFn::OnStart,
                        HookFn(|world| world.insert(HookFnValue(1))),
                    )
                    .with_hook_fn(
                        HookableFn::OnStop,
                        HookFn(|world| world.insert(HookFnValue(2))),
                    )
                    .with_hook_fn(
                        HookableFn::OnPause,
                        HookFn(|world| world.insert(HookFnValue(4))),
                    )
                    .with_hook_fn(
                        HookableFn::OnResume,
                        HookFn(|world| world.insert(HookFnValue(8))),
                    );
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
        fn on_start(&mut self, mut _data: StateData<'_, GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnStart);
        }
        fn on_stop(&mut self, _data: StateData<'_, GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnStop);
        }
        fn on_pause(&mut self, _data: StateData<'_, GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnPause);
        }

        fn on_resume(&mut self, _data: StateData<'_, GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnResume);
        }

        fn handle_event(
            &mut self,
            _data: StateData<'_, GameData<'a, 'b>>,
            _event: AppEvent,
        ) -> Trans<GameData<'a, 'b>, AppEvent> {
            self.invocations.borrow_mut().push(Invocation::HandleEvent);
            Trans::None
        }

        fn fixed_update(
            &mut self,
            _data: StateData<'_, GameData<'a, 'b>>,
        ) -> Trans<GameData<'a, 'b>, AppEvent> {
            self.invocations.borrow_mut().push(Invocation::FixedUpdate);
            Trans::None
        }

        fn update(
            &mut self,
            _data: StateData<'_, GameData<'a, 'b>>,
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
    #[derive(Debug, PartialEq)]
    struct HookFnValue(u32);

    #[derive(Debug)]
    struct SystemCounter;
    impl<'s> System<'s> for SystemCounter {
        type SystemData = WriteExpect<'s, Counter>;
        fn run(&mut self, mut counter: Self::SystemData) {
            *counter = Counter((*counter).0 + 10);
        }

        fn setup(&mut self, world: &mut World) {
            Self::SystemData::setup(world);
            world.insert(Counter(0));
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
