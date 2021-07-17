use amethyst::{
    ecs::{Entity, World, WorldExt},
    input::{is_key_down, VirtualKeyCode},
    GameData, State, StateData, Trans,
};
use application_event::AppEvent;
use derivative::Derivative;
use derive_new::new;
use game_model::play::GameEntities;
use game_play_model::{GamePlayEntity, GamePlayEvent, GamePlayStatus};
use log::debug;
use state_registry::StateId;
use state_support::StateEntityUtils;

/// `State` where game play takes place.
#[derive(Derivative, Default, new)]
#[derivative(Debug)]
pub struct GamePlayState;

impl GamePlayState {
    fn terminate_entities(&mut self, world: &mut World) {
        // This `allow` is needed because rustc evaluates that `game_entities` does not
        // live long enough when entities is constructed, so we need to bind
        // entities to a variable.
        //
        // However, that triggers the clippy lint that we're binding and then returning.
        // Pending:
        //
        // * <https://github.com/rust-lang-nursery/rust-clippy/issues/1524>
        // * <https://github.com/rust-lang-nursery/rust-clippy/issues/2904>
        #[allow(clippy::let_and_return)]
        let entities = {
            let mut game_entities = world.write_resource::<GameEntities>();
            let entities = game_entities.drain().collect::<Vec<Entity>>();

            entities
        };
        entities.into_iter().for_each(|entity| {
            world
                .delete_entity(entity)
                .expect("Failed to delete game entity.");
        });

        StateEntityUtils::clear::<GamePlayEntity>(world);
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, AppEvent> for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.insert(StateId::GamePlay);
        data.world.insert(GamePlayStatus::Playing);
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.terminate_entities(&mut data.world);
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        data.world.insert(StateId::GamePlay);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: AppEvent,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        match event {
            AppEvent::Window(window_event) => {
                if is_key_down(&window_event, VirtualKeyCode::Escape) {
                    debug!("Returning from `GamePlayState`.");
                    data.world.insert(GamePlayStatus::None);
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            AppEvent::GamePlay(game_play_event) => {
                match game_play_event {
                    GamePlayEvent::Return => {
                        debug!("Returning from `GamePlayState`.");
                        data.world.insert(GamePlayStatus::None);
                        Trans::Pop
                    }
                    GamePlayEvent::Restart => {
                        // TODO: Switch to `GameLoadingState`
                        Trans::None
                    }
                    GamePlayEvent::Pause => {
                        data.world.insert(GamePlayStatus::Paused);
                        Trans::None
                    }
                    GamePlayEvent::Resume => {
                        data.world.insert(GamePlayStatus::Playing);
                        Trans::None
                    }
                    GamePlayEvent::End => Trans::None,
                    GamePlayEvent::EndStats => {
                        // TODO: `GamePlayStats` state.
                        Trans::Pop
                    }
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        // Note: The built-in dispatcher must be run before the state specific
        // dispatcher as the `"input_system"` is registered in the main
        // dispatcher, and is a dependency of the `ControllerInputUpdateSystem`.
        data.data.update(&data.world);
        Trans::None
    }
}
