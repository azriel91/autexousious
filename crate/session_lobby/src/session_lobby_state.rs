use amethyst::{GameData, State, StateData, Trans};
use application_event::AppEvent;
use application_state::{AppState, AppStateBuilder};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use session_lobby_model::{SessionLobbyEntity, SessionLobbyEvent};
use state_registry::StateId;

/// `State` that displays the devices in the same online session.
///
/// This state is not intended to be constructed directly, but through the
/// [`SessionLobbyStateBuilder`][state_builder].
///
/// [state_builder]: session_lobby_state/struct.SessionLobbyStateBuilder.html
pub type SessionLobbyState =
    AppState<'static, 'static, SessionLobbyStateDelegate, SessionLobbyEntity>;

/// Builder for a `SessionLobbyState`.
///
/// `SystemBundle`s to run in the `SessionLobbyState`'s dispatcher are registered on this builder.
pub type SessionLobbyStateBuilder =
    AppStateBuilder<'static, 'static, SessionLobbyStateDelegate, SessionLobbyEntity>;

/// Delegate `State` for the session lobby.
///
/// This state is not intended to be used directly, but wrapped in an `AppState`. The
/// `SessionLobbyState` is an alias with this as a delegate state.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct SessionLobbyStateDelegate;

impl SessionLobbyStateDelegate {
    fn initialize_state(data: StateData<'_, GameData<'static, 'static>>) {
        data.world.insert(StateId::SessionLobby);
    }
}

impl State<GameData<'static, 'static>, AppEvent> for SessionLobbyStateDelegate {
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
        if let AppEvent::SessionLobby(session_lobby_event) = event {
            debug!("Received session_lobby_event: {:?}", session_lobby_event);
            match session_lobby_event {
                SessionLobbyEvent::Back => Trans::Pop,
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }
}
