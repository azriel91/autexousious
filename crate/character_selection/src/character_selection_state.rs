use std::fmt::Debug;

use amethyst::{core::SystemBundle, ecs::prelude::*, prelude::*, shrev::EventChannel};
use application_event::AppEvent;
use application_state::AppState;
use character_selection_model::{
    CharacterSelectionEvent, CharacterSelections, CharacterSelectionsStatus,
};

use CharacterSelectionBundle;

/// `State` where character selection takes place.
///
/// This state is not intended to be constructed directly, but through the
/// [`CharacterSelectionStateBuilder`][state_builder].
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after character selection is complete.
/// * `S`: State to return.
/// * `E`: Custom event type that states can respond to.
///
/// [state_builder]: character_selection_state/struct.CharacterSelectionStateBuilder.html
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct CharacterSelectionState<'a, 'b, F, S>
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

impl<'a, 'b, F, S> CharacterSelectionState<'a, 'b, F, S>
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

    fn initialize_character_selections(&mut self, world: &mut World) {
        let mut selections_status = world.write_resource::<CharacterSelectionsStatus>();
        *selections_status = CharacterSelectionsStatus::Waiting;
    }
}

impl<'a, 'b, F, S> State<GameData<'a, 'b>, AppEvent> for CharacterSelectionState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AppState<'a, 'b> + 'static,
{
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.initialize_dispatcher(&mut data.world);
        self.initialize_character_selections(&mut data.world);
    }

    fn on_stop(&mut self, _data: StateData<GameData<'a, 'b>>) {
        self.terminate_dispatcher();
    }

    fn on_resume(&mut self, data: StateData<GameData<'a, 'b>>) {
        let mut selections_status = data.world.write_resource::<CharacterSelectionsStatus>();
        *selections_status = CharacterSelectionsStatus::Confirmed;
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
        event: StateEvent<AppEvent>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        match event {
            StateEvent::Custom(app_event) => match app_event {
                AppEvent::CharacterSelection(character_selection_event) => {
                    debug!(
                        "Received character_selection_event: {:?}",
                        character_selection_event
                    );
                    let mut channel = data
                        .world
                        .write_resource::<EventChannel<CharacterSelectionEvent>>();
                    channel.single_write(character_selection_event);
                }
                _ => {}
            },
            _ => {}
        }
        Trans::None
    }

    fn fixed_update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        // Note: The built-in dispatcher must be run before the state specific dispatcher as the
        // `"input_system"` is registered in the main dispatcher, and is a dependency of the
        // `CharacterSelectionSystem`.
        data.data.update(&data.world);
        self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);

        let selections_status = data.world.read_resource::<CharacterSelectionsStatus>();
        if *selections_status == CharacterSelectionsStatus::Ready {
            let character_selections = data.world.read_resource::<CharacterSelections>();
            info!(
                "character_selections: `{:?}`",
                &character_selections.selections
            );

            // TODO: `Trans:Push` when we have a proper character selection menu.
            Trans::Switch((self.next_state_fn)())
        } else {
            Trans::None
        }
    }
}

/// `State` where character selection takes place.
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after character selection is complete.
/// * `S`: State to return.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct CharacterSelectionStateBuilder<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AppState<'a, 'b> + 'static,
{
    /// State specific dispatcher builder.
    #[derivative(Debug = "ignore")]
    #[new(value = "DispatcherBuilder::new()")]
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
    /// System names that the `CharacterSelectionSystem` should depend on.
    #[new(default)]
    character_selection_system_dependencies: Option<Vec<String>>,
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: F,
}

impl<'a, 'b, F, S> CharacterSelectionStateBuilder<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AppState<'a, 'b> + 'static,
{
    /// Registers a bundle whose systems to run in the `CharacterSelectionState`.
    ///
    /// # Parameters
    ///
    /// * `bundle`: Bundle to register.
    pub fn with_bundle<B: SystemBundle<'a, 'b>>(mut self, bundle: B) -> Self {
        bundle
            .build(&mut self.dispatcher_builder)
            .expect("Failed to register bundle for `CharacterSelectionState`.");
        self
    }

    /// Specifies system dependencies for the `CharacterSelectionSystem`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.character_selection_system_dependencies = Some(dependencies);
        self
    }

    /// Builds and returns the `CharacterSelectionState`.
    pub fn build(mut self) -> CharacterSelectionState<'a, 'b, F, S> {
        let mut bundle = CharacterSelectionBundle::new();

        if let Some(deps) = self.character_selection_system_dependencies {
            bundle = bundle.with_system_dependencies(&deps);
        }

        bundle
            .build(&mut self.dispatcher_builder)
            .expect("Failed to register `CharacterSelectionBundle` with dispatcher.");

        CharacterSelectionState::new(Some(self.dispatcher_builder), self.next_state_fn)
    }
}
