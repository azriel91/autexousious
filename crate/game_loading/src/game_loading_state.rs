use std::fmt::Debug;
use std::marker::PhantomData;

use amethyst::{
    core::SystemBundle, ecs::prelude::*, input::is_key_down, prelude::*, renderer::VirtualKeyCode,
};
use game_model::play::GameEntities;

use GameLoadingBundle;

/// `State` where game play takes place.
#[derive(Derivative, Default, new)]
#[derivative(Debug)]
pub struct GameLoadingState<'a, 'b, F, S, E>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>, E> + 'static,
    E: Send + Sync + 'static,
{
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: Box<F>,
    /// State specific dispatcher.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    dispatcher: Option<Dispatcher<'static, 'static>>,
    /// Data type used by this state and the returned state (see `StateData`).
    state_data: PhantomData<(GameData<'a, 'b>, E)>,
}

impl<'a, 'b, F, S, E> GameLoadingState<'a, 'b, F, S, E>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>, E> + 'static,
    E: Send + Sync + 'static,
{
    /// Sets up the dispatcher for this state.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to operate on.
    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();

        GameLoadingBundle::new()
            .build(&mut dispatcher_builder)
            .expect("Failed to register `GameLoadingBundle`.");

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut world.res);
        self.dispatcher = Some(dispatcher);
    }

    /// Removes all entities stored in the `GameEntities` resource.
    ///
    /// Since the population of `GameEntities` is used to determine whether the next state should be
    /// switched to, we need to clear it when the entities are stale.
    fn clear_game_entities(&mut self, world: &mut World) {
        let mut game_entities = world.write_resource::<GameEntities>();
        game_entities.objects.clear();
        game_entities.map_layers.clear();
    }

    /// Terminates the dispatcher.
    fn terminate_dispatcher(&mut self) {
        self.dispatcher = None;
    }
}

impl<'a, 'b, F, S, E> State<GameData<'a, 'b>, E> for GameLoadingState<'a, 'b, F, S, E>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>, E> + 'static,
    E: Send + Sync + 'static,
{
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.initialize_dispatcher(&mut data.world);
        self.clear_game_entities(&mut data.world);
    }

    fn on_stop(&mut self, _data: StateData<GameData<'a, 'b>>) {
        self.terminate_dispatcher();
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        self.clear_game_entities(&mut data.world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: StateEvent<E>,
    ) -> Trans<GameData<'a, 'b>, E> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                info!("Returning from `GameLoadingState`.");
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, E> {
        data.data.update(&data.world);
        self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);

        let game_entities = &data.world.read_resource::<GameEntities>();
        if !game_entities.objects.is_empty() && !game_entities.map_layers.is_empty() {
            // TODO: `Trans:Push` when we have a proper map selection menu.
            Trans::Switch((self.next_state_fn)())
        } else {
            Trans::None
        }
    }
}
