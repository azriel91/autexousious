use std::fmt::Debug;
use std::marker::PhantomData;

use amethyst::{core::SystemBundle, ecs::prelude::*, prelude::*};
use application_event::AppEvent;
use application_state::AppState;
use map_selection_model::MapSelection;

use MapSelectionBundle;
use MapSelectionStatus;

/// `State` where map selection takes place.
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after map selection is complete.
/// * `S`: State to return.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct MapSelectionState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AppState<'a, 'b> + 'static,
{
    /// State specific dispatcher builder.
    #[derivative(Debug = "ignore")]
    dispatcher_builder: Option<DispatcherBuilder<'a, 'b>>,
    /// State specific dispatcher.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    dispatcher: Option<Dispatcher<'a, 'b>>,
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: F,
}

impl<'a, 'b, F, S> MapSelectionState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AppState<'a, 'b> + 'static,
{
    /// Sets up the dispatcher for this state.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to operate on.
    fn initialize_dispatcher(&mut self, world: &mut World) {
        if self.dispatcher.is_none() {
            let mut dispatcher = self
                .dispatcher_builder
                .take()
                .expect(
                    "Expected `dispatcher_builder` to exist when `dispatcher` is not yet built.",
                ).build();
            dispatcher.setup(&mut world.res);
            self.dispatcher = Some(dispatcher);
        }
    }

    /// Terminates the dispatcher.
    fn terminate_dispatcher(&mut self) {
        self.dispatcher = None;
    }

    fn reset_map_selection_state(&self, world: &mut World) {
        let mut map_selection_status = world.write_resource::<MapSelectionStatus>();
        *map_selection_status = MapSelectionStatus::Pending;
    }
}

impl<'a, 'b, F, S> State<GameData<'a, 'b>, AppEvent> for MapSelectionState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AppState<'a, 'b> + 'static,
{
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.initialize_dispatcher(&mut data.world);
        self.reset_map_selection_state(&mut data.world);
    }

    fn on_stop(&mut self, _data: StateData<GameData<'a, 'b>>) {
        self.terminate_dispatcher();
    }

    fn on_resume(&mut self, data: StateData<GameData<'a, 'b>>) {
        self.reset_map_selection_state(data.world);
    }

    fn fixed_update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        data.data.update(&data.world);
        self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);

        let map_selection_status = data.world.read_resource::<MapSelectionStatus>();
        if *map_selection_status == MapSelectionStatus::Confirmed {
            let map_selection = data.world.read_resource::<MapSelection>();

            info!("Map selection: `{}`", *map_selection);

            // TODO: `Trans:Push` when we have a proper map selection menu.
            Trans::Switch((self.next_state_fn)())
        } else {
            Trans::None
        }
    }
}

/// Builder for the `MapSelectionState`.
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after map selection is complete.
/// * `S`: State to return.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct MapSelectionStateBuilder<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AppState<'a, 'b> + 'static,
{
    /// State specific dispatcher builder.
    #[derivative(Debug = "ignore")]
    #[new(value = "DispatcherBuilder::new()")]
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
    /// System names that the `MapSelectionSystem` should depend on.
    #[new(default)]
    map_selection_system_dependencies: Option<Vec<String>>,
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: F,
    /// Data type used by the state and the returned state (see `StateData`).
    game_data: PhantomData<(GameData<'a, 'b>, AppEvent)>,
}

impl<'a, 'b, F, S> MapSelectionStateBuilder<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AppState<'a, 'b> + 'static,
{
    /// Registers a bundle whose systems to run in the `MapSelectionState`.
    ///
    /// # Parameters
    ///
    /// * `bundle`: Bundle to register.
    pub fn with_bundle<B: SystemBundle<'a, 'b>>(mut self, bundle: B) -> Self {
        bundle
            .build(&mut self.dispatcher_builder)
            .expect("Failed to register bundle for `MapSelectionState`.");
        self
    }

    /// Specifies system dependencies for the `MapSelectionSystem`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.map_selection_system_dependencies = Some(dependencies);
        self
    }

    /// Builds and returns the `MapSelectionState`.
    pub fn build(mut self) -> MapSelectionState<'a, 'b, F, S> {
        let mut bundle = MapSelectionBundle::new();

        if let Some(deps) = self.map_selection_system_dependencies {
            bundle = bundle.with_system_dependencies(&deps);
        }

        bundle
            .build(&mut self.dispatcher_builder)
            .expect("Failed to register `MapSelectionBundle` with dispatcher.");

        MapSelectionState::new(Some(self.dispatcher_builder), self.next_state_fn)
    }
}
