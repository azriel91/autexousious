use std::fmt::Debug;
use std::marker::PhantomData;

use amethyst::{core::SystemBundle, ecs::prelude::*, prelude::*};

use CharacterSelection;
use CharacterSelectionBundle;

/// `State` where character selection takes place.
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after character selection is complete.
/// * `S`: State to return.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct CharacterSelectionState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>> + 'static,
{
    /// State specific dispatcher.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    dispatcher: Option<Dispatcher<'static, 'static>>,
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: Box<F>,
    /// Data type used by this state and the returned state (see `StateData`).
    state_data: PhantomData<GameData<'a, 'b>>,
}

impl<'a, 'b, F, S> CharacterSelectionState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>> + 'static,
{
    /// Sets up the dispatcher for this state.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to operate on.
    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();

        CharacterSelectionBundle::new()
            .build(&mut dispatcher_builder)
            .expect("Failed to register `CharacterSelectionBundle`.");

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut world.res);
        self.dispatcher = Some(dispatcher);
    }

    /// Terminates the dispatcher.
    fn terminate_dispatcher(&mut self) {
        self.dispatcher = None;
    }
}

impl<'a, 'b, F, S> State<GameData<'a, 'b>> for CharacterSelectionState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>> + 'static,
{
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.initialize_dispatcher(&mut data.world);
    }

    fn on_stop(&mut self, _data: StateData<GameData>) {
        self.terminate_dispatcher();
    }

    fn fixed_update(&mut self, data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>> {
        // Note: The built-in dispatcher must be run before the state specific dispatcher as the
        // `"input_system"` is registered in the main dispatcher, and is a dependency of the
        // `CharacterSelectionSystem`.
        data.data.update(&data.world);
        self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);

        let selected_characters = data.world.read_resource::<CharacterSelection>();
        if selected_characters.is_empty() {
            Trans::None
        } else {
            info!("selected_characters: `{:?}`", &*selected_characters);

            // TODO: `Trans:Push` when we have a proper character selection menu.
            Trans::Switch((self.next_state_fn)())
        }
    }
}
