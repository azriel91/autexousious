use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use amethyst::{
    core::{
        deferred_dispatcher_operation::{AddBundle, AddSystem, DispatcherOperation},
        SystemBundle,
    },
    ecs::{Component, Dispatcher, DispatcherBuilder, System, World, WorldExt},
    GameData, State, StateData, Trans,
};
use application_event::AppEvent;
use derivative::Derivative;
use derive_new::new;
use log::debug;
use state_support::StateEntityUtils;

use crate::{AutexState, HookFn, HookableFn};

/// Wrapper `State` with a custom dispatcher.
///
/// All `State` methods are called through, with special handling for the
/// following:
///
/// * `on_start`: The `World` is setup with the state specific dispatcher before
///   calling through.
/// * `update`: The game data and state specific dispatchers are run before
///   calling through.
///
/// This state is not intended to be constructed directly, but through the
/// [`AppStateBuilder`][state_builder].
///
/// # Type Parameters
///
/// * `S`: `State` to delegate to.
/// * `I`: `State` identifier component to identify entities to delete when the
///   state is popped.
///
/// [state_builder]: application_state/struct.AppStateBuilder.html
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct AppState<'a, 'b, S, I>
where
    S: AutexState<'a, 'b>,
    I: Component,
    I::Storage: Default,
{
    /// Functions to instantiate state specific dispatcher systems.
    #[derivative(Debug = "ignore")]
    dispatcher_operations: Option<Vec<Box<dyn DispatcherOperation<'a, 'b> + 'a>>>,
    /// State specific dispatcher.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    dispatcher: Option<Dispatcher<'a, 'b>>,
    /// The `State` to delegate to.
    #[derivative(Debug(bound = "S: Debug"))]
    delegate: S,
    /// Functions to run at the beginning of the various `State` methods.
    hook_fns: HashMap<HookableFn, Vec<HookFn>>,
    /// `PhantomData` for state tag component.
    marker: PhantomData<I>,
}

impl<'a, 'b, S, I> AppState<'a, 'b, S, I>
where
    S: AutexState<'a, 'b>,
    I: Component,
    I::Storage: Default,
{
    /// Sets up the dispatcher for this state.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to operate on.
    fn initialize_dispatcher(&mut self, world: &mut World) {
        if self.dispatcher.is_none() {
            let dispatcher_operations = self.dispatcher_operations.take().expect(
                "Expected `dispatcher_operations` to exist when dispatcher is not yet built.",
            );

            let mut dispatcher_builder = DispatcherBuilder::new();
            dispatcher_operations
                .into_iter()
                .for_each(|dispatcher_operation| {
                    dispatcher_operation
                        .exec(world, &mut dispatcher_builder)
                        .expect("Failed to execute dispatcher operation.");
                });

            let mut dispatcher = dispatcher_builder.build();
            dispatcher.setup(world);
            self.dispatcher = Some(dispatcher);
        }
    }
}

impl<'a, 'b, S, I> State<GameData<'a, 'b>, AppEvent> for AppState<'a, 'b, S, I>
where
    S: AutexState<'a, 'b>,
    I: Component,
    I::Storage: Default,
{
    fn on_start(&mut self, mut data: StateData<'_, GameData<'a, 'b>>) {
        data.world.register::<I>();

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

        StateEntityUtils::clear::<I>(&mut data.world);

        self.delegate.on_stop(data);
    }

    fn on_pause(&mut self, mut data: StateData<'_, GameData<'a, 'b>>) {
        if let Some(ref functions) = self.hook_fns.get(&HookableFn::OnPause) {
            functions
                .iter()
                .for_each(|function| function(&mut data.world));
        }

        StateEntityUtils::clear::<I>(&mut data.world);

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
        // Note: The built-in dispatcher must be run before the state specific
        // dispatcher as the `"input_system"` is registered in the main
        // dispatcher, and by design we have chosen that systems that depend on
        // that should be placed in the state specific dispatcher.
        data.data.update(&data.world);
        self.dispatcher
            .as_mut()
            .expect("Expected `dispatcher` to be set up.")
            .dispatch(&data.world);

        self.delegate.update(data)
    }
}

/// Builder for an `AppState`.
///
/// `SystemBundle`s to run in the `AppState`'s dispatcher are registered on this
/// builder.
///
/// # Type Parameters
///
/// * `S`: `State` to delegate to.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct AppStateBuilder<'a, 'b, S, I>
where
    S: AutexState<'a, 'b>,
    I: Component,
    I::Storage: Default,
{
    /// Functions to instantiate state specific dispatcher systems.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b> + 'a>>,
    /// The `State` to delegate to.
    #[derivative(Debug(bound = "S: Debug"))]
    delegate: S,
    /// Functions to run at the beginning of the various `State` methods.
    #[new(default)]
    hook_fns: HashMap<HookableFn, Vec<HookFn>>,
    /// Marker for component to identify entities to delete.
    marker: PhantomData<I>,
}

impl<'a, 'b, S, I> AppStateBuilder<'a, 'b, S, I>
where
    S: AutexState<'a, 'b>,
    I: Component,
    I::Storage: Default,
{
    /// Registers a bundle whose systems to run in the `AppState`.
    ///
    /// # Parameters
    ///
    /// * `bundle`: Bundle to register.
    pub fn with_bundle<B>(mut self, bundle: B) -> Self
    where
        B: SystemBundle<'a, 'b> + 'a,
    {
        self.dispatcher_operations
            .push(Box::new(AddBundle { bundle }));
        self
    }

    /// Registers a function to be run at the beginning of a particular `State`
    /// method.
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

    /// Registers a `System` with the dispatcher builder.
    ///
    /// # Parameters
    ///
    /// * `system`: Function to instantiate the `System`.
    /// * `name`: Name to register the system with, used for dependency
    ///   ordering.
    /// * `deps`: Names of systems that must run before this system.
    pub fn with_system<Sys, N>(mut self, system: Sys, name: N, dependencies: &[N]) -> Self
    where
        Sys: for<'c> System<'c> + Send + 'a,
        N: Into<String> + Clone,
    {
        let name = Into::<String>::into(name);
        let dependencies = dependencies
            .iter()
            .map(Clone::clone)
            .map(Into::<String>::into)
            .collect::<Vec<String>>();
        let dispatcher_operation = Box::new(AddSystem {
            system,
            name,
            dependencies,
        }) as Box<dyn DispatcherOperation<'a, 'b> + 'a>;
        self.dispatcher_operations.push(dispatcher_operation);
        self
    }

    /// Builds and returns the `AppState`.
    pub fn build(self) -> AppState<'a, 'b, S, I> {
        AppState::new(
            Some(self.dispatcher_operations),
            self.delegate,
            self.hook_fns,
        )
    }
}
