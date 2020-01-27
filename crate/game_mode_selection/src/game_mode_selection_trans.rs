use std::any;

use amethyst::{GameData, Trans};
use application_event::AppEvent;
use character_selection::{
    CharacterSelectionStateBuilder, CharacterSelectionStateDelegate, CharacterSelectionSystem,
};
use control_settings::ControlSettingsState;
use game_loading::GameLoadingState;
use game_mode_selection_model::GameModeIndex;
use game_play::GamePlayState;
use map_selection::{MapSelectionBundle, MapSelectionStateBuilder, MapSelectionStateDelegate};

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
                // kcov-ignore-start
                let game_play_fn = || Box::new(GamePlayState::new());
                let game_loading_fn = move || Box::new(GameLoadingState::new(game_play_fn));
                let map_selection_fn = move || {
                    let state = MapSelectionStateBuilder::new(MapSelectionStateDelegate::new(
                        game_loading_fn,
                    ))
                    .with_bundle(MapSelectionBundle::new())
                    .build();

                    Box::new(state)
                };
                // kcov-ignore-end
                let character_selection_state = {
                    let state = CharacterSelectionStateBuilder::new(
                        CharacterSelectionStateDelegate::new(map_selection_fn),
                    )
                    .with_system(
                        CharacterSelectionSystem::new(),
                        any::type_name::<CharacterSelectionSystem>(),
                        &[],
                    )
                    .build();

                    Box::new(state)
                };

                Trans::Push(character_selection_state)
            }
            GameModeIndex::ControlSettings => Trans::Push(Box::new(ControlSettingsState::new())),
            GameModeIndex::Exit => Trans::Quit,
        }
    } // kcov-ignore
}
