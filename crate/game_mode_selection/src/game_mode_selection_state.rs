use amethyst::{GameData, State, StateData, Trans};
use application_event::AppEvent;
use application_menu::MenuEvent;
use application_state::{AppState, AppStateBuilder};
use derivative::Derivative;
use derive_new::new;
use game_mode_selection_model::GameModeSelectionEntity;
use log::debug;
use state_registry::StateId;

use crate::GameModeSelectionTrans;

/// `State` where game mode selection takes place.
///
/// This state is not intended to be constructed directly, but through the
/// [`GameModeSelectionStateBuilder`][state_builder].
///
/// [state_builder]: game_mode_selection_state/struct.GameModeSelectionStateBuilder.html
pub type GameModeSelectionState =
    AppState<'static, 'static, GameModeSelectionStateDelegate, GameModeSelectionEntity>;

/// Builder for a `GameModeSelectionState`.
///
/// `SystemBundle`s to run in the `GameModeSelectionState`'s dispatcher are registered on this
/// builder.
pub type GameModeSelectionStateBuilder =
    AppStateBuilder<'static, 'static, GameModeSelectionStateDelegate, GameModeSelectionEntity>;

/// Delegate `State` for game mode selection.
///
/// This state is not intended to be used directly, but wrapped in an `AppState`. The
/// `GameModeSelectionState` is an alias with this as a delegate state.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct GameModeSelectionStateDelegate;

impl GameModeSelectionStateDelegate {
    fn initialize_state(data: StateData<'_, GameData<'static, 'static>>) {
        data.world.insert(StateId::GameModeSelection);
    }
}

impl State<GameData<'static, 'static>, AppEvent> for GameModeSelectionStateDelegate {
    fn on_start(&mut self, data: StateData<'_, GameData<'static, 'static>>) {
        Self::initialize_state(data);
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'static, 'static>>) {
        Self::initialize_state(data);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'static, 'static>>,
        event: AppEvent,
    ) -> Trans<GameData<'static, 'static>, AppEvent> {
        if let AppEvent::GameModeSelection(game_mode_selection_event) = event {
            debug!(
                "Received game_mode_selection_event: {:?}",
                game_mode_selection_event
            );
            match game_mode_selection_event {
                MenuEvent::Select(idx) => GameModeSelectionTrans::trans(idx),
                MenuEvent::Close => Trans::Pop,
            }
        } else {
            Trans::None
        }
    }
}
