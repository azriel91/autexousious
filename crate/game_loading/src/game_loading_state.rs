use std::fmt::Debug;

use amethyst::{core::SystemBundle, ecs::prelude::*, prelude::*};
use application_event::AppEvent;
use application_state::AutexState;
use derivative::Derivative;
use derive_new::new;
use game_model::play::GameEntities;
use state_registry::StateId;

use crate::{GameLoadingBundle, GameLoadingStatus};

/// `State` where game play takes place.
#[derive(Derivative, Default, new)]
#[derivative(Debug)]
pub struct GameLoadingState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    /// State specific dispatcher.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    dispatcher: Option<Dispatcher<'a, 'b>>,
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: F,
}

impl<'a, 'b, F, S> GameLoadingState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    /// Sets up the dispatcher for this state.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to operate on.
    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();

        GameLoadingBundle::new()
            .build(world, &mut dispatcher_builder)
            .expect("Failed to register `GameLoadingBundle`.");

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
    }

    /// Removes all entities stored in the `GameEntities` resource.
    ///
    /// Since the population of `GameEntities` is used to determine whether the
    /// next state should be switched to, we need to clear it when the
    /// entities are stale.
    fn reset_game_loading_status(&mut self, world: &mut World) {
        let mut game_entities = world.write_resource::<GameEntities>();
        game_entities.objects.clear();
        game_entities.map_layers.clear();

        world.write_resource::<GameLoadingStatus>().reset();
    }

    /// Terminates the dispatcher.
    fn terminate_dispatcher(&mut self) {
        self.dispatcher = None;
    }
}

impl<'a, 'b, F, S> State<GameData<'a, 'b>, AppEvent> for GameLoadingState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world.insert(StateId::GameLoading);

        self.initialize_dispatcher(&mut data.world);
        self.reset_game_loading_status(&mut data.world);
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'a, 'b>>) {
        self.terminate_dispatcher();
    }

    fn on_resume(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world.insert(StateId::GameLoading);

        self.reset_game_loading_status(&mut data.world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        _event: AppEvent,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        data.data.update(&data.world);
        self.dispatcher.as_mut().unwrap().dispatch(&data.world);

        if data.world.read_resource::<GameLoadingStatus>().loaded() {
            // TODO: `Trans:Push` when we have a proper map selection menu.
            Trans::Switch((self.next_state_fn)())
        } else {
            Trans::None
        }
    }
}
