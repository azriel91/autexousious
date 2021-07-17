use std::any;

use amethyst::{GameData, State, Trans};
use application_event::AppEvent;
use character_selection::{
    CharacterSelectionStateBuilder, CharacterSelectionStateDelegate, CharacterSelectionSystem,
};
use control_settings::ControlSettingsState;
use game_loading::GameLoadingState;
use game_mode_selection_model::GameModeIndex;
use game_play::GamePlayState;
use map_selection::{MapSelectionBundle, MapSelectionStateBuilder, MapSelectionStateDelegate};
use network_mode_selection::{NetworkModeSelectionStateBuilder, NetworkModeSelectionStateDelegate};

/// Returns the `Trans` for a given `GameModeIndex`.
#[derive(Debug)]
pub struct GameModeSelectionTrans;

impl GameModeSelectionTrans {
    /// Returns a transition when a menu item has been selected.
    ///
    /// # Parameters
    ///
    /// * `game_mode_index`: The selected index.
    pub fn trans(game_mode_index: GameModeIndex) -> Trans<GameData<'static, 'static>, AppEvent> {
        match game_mode_index {
            GameModeIndex::StartGame => {
                let character_selection_state = Self::character_selection_state();
                Trans::Push(character_selection_state)
            }
            GameModeIndex::NetworkPlay => {
                let network_mode_selection_state = Self::network_mode_selection_state();
                Trans::Push(network_mode_selection_state)
            }
            GameModeIndex::ControlSettings => Trans::Push(Box::new(ControlSettingsState::new())),
            GameModeIndex::Exit => Trans::Quit,
        }
    }

    // kcov-ignore

    fn character_selection_state() -> Box<dyn State<GameData<'static, 'static>, AppEvent>> {
        // kcov-ignore-start
        let game_play_fn = || Box::new(GamePlayState::new());
        let game_loading_fn = move || Box::new(GameLoadingState::new(game_play_fn));
        let map_selection_fn = move || {
            let state =
                MapSelectionStateBuilder::new(MapSelectionStateDelegate::new(game_loading_fn))
                    .with_bundle(MapSelectionBundle::new())
                    .build();

            Box::new(state)
        };
        // kcov-ignore-end
        let state = CharacterSelectionStateBuilder::new(CharacterSelectionStateDelegate::new(
            map_selection_fn,
        ))
        .with_system(
            CharacterSelectionSystem::new(),
            any::type_name::<CharacterSelectionSystem>(),
            &[],
        )
        .build();

        Box::new(state)
    }

    fn network_mode_selection_state() -> Box<dyn State<GameData<'static, 'static>, AppEvent>> {
        let state =
            NetworkModeSelectionStateBuilder::new(NetworkModeSelectionStateDelegate::new()).build();

        Box::new(state)
    }
}
