use asset_selection_model::config::AssetSelectionEventCommand;
use control_settings_model::ControlSettingsEvent;
use derive_more::From;
use game_mode_selection_model::GameModeSelectionEventArgs;
use game_play_model::GamePlayEventArgs;
use network_mode_selection_model::NetworkModeSelectionEventArgs;
use serde::{Deserialize, Serialize};
use session_host_model::config::SessionHostEventCommand;
use session_join_model::config::SessionJoinEventCommand;
use session_lobby_model::config::SessionLobbyEventCommand;

/// Configuration type to indicate what `AppEvent` to send as part of an `InputReaction`.
///
/// Note:
///
/// * `ControlInputEvent`s are skipped as this is used to indicate events sent upon control input.
/// * `StdioCommandEvent`s are skipped as those events are not intended to be sent through UI items.
#[derive(Clone, Copy, Debug, Deserialize, From, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum InputReactionAppEvent {
    /// `asset_selection` events.
    AssetSelection(AssetSelectionEventCommand),
    /// `control_settings` events.
    ControlSettings(ControlSettingsEvent),
    /// `game_mode_selection` events.
    GameModeSelection(GameModeSelectionEventArgs),
    /// `game_play` events.
    GamePlay(GamePlayEventArgs),
    /// `session_host` events.
    SessionHost(SessionHostEventCommand),
    /// `session_join` events.
    SessionJoin(SessionJoinEventCommand),
    /// `session_lobby` events.
    SessionLobby(SessionLobbyEventCommand),
    /// `network_mode_selection` events.
    NetworkModeSelection(NetworkModeSelectionEventArgs),
}
