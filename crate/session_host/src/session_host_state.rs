use amethyst::{GameData, State, StateData, Trans};
use application_event::AppEvent;
use application_state::{AppState, AppStateBuilder};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use network_session_model::play::SessionStatus;
use session_host_model::{SessionHostEntity, SessionHostEvent};
use session_lobby::{SessionLobbyStateBuilder, SessionLobbyStateDelegate};
use state_registry::StateId;

/// `State` where session hosting takes place.
///
/// This state is not intended to be constructed directly, but through the
/// [`SessionHostStateBuilder`][state_builder].
///
/// [state_builder]: session_host_state/struct.SessionHostStateBuilder.html
pub type SessionHostState = AppState<'static, 'static, SessionHostStateDelegate, SessionHostEntity>;

/// Builder for a `SessionHostState`.
///
/// `SystemBundle`s to run in the `SessionHostState`'s dispatcher are registered on this builder.
pub type SessionHostStateBuilder =
    AppStateBuilder<'static, 'static, SessionHostStateDelegate, SessionHostEntity>;

/// Delegate `State` for session hosting.
///
/// This state is not intended to be used directly, but wrapped in an `AppState`. The
/// `SessionHostState` is an alias with this as a delegate state.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct SessionHostStateDelegate;

impl SessionHostStateDelegate {
    fn initialize_state(data: StateData<'_, GameData<'static, 'static>>) {
        data.world.insert(StateId::SessionHost);
        data.world.insert(SessionStatus::None);
    }
}

impl State<GameData<'static, 'static>, AppEvent> for SessionHostStateDelegate {
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
        if let AppEvent::SessionHost(session_host_event) = event {
            debug!("Received session_host_event: {:?}", session_host_event);
            match session_host_event {
                SessionHostEvent::SessionAccept(_) => {
                    let session_lobby_state =
                        SessionLobbyStateBuilder::new(SessionLobbyStateDelegate::new()).build();
                    Trans::Push(Box::new(session_lobby_state))
                }
                SessionHostEvent::Back => Trans::Pop,
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }
}
