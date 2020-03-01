use amethyst::{GameData, State, StateData, Trans};
use application_event::AppEvent;
use application_state::{AppState, AppStateBuilder};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use session_join_model::{SessionJoinEntity, SessionJoinEvent};
use state_registry::StateId;

/// `State` where session joining takes place.
///
/// This state is not intended to be constructed directly, but through the
/// [`SessionJoinStateBuilder`][state_builder].
///
/// [state_builder]: network_mode_selection_state/struct.SessionJoinStateBuilder.html
pub type SessionJoinState = AppState<'static, 'static, SessionJoinStateDelegate, SessionJoinEntity>;

/// Builder for a `SessionJoinState`.
///
/// `SystemBundle`s to run in the `SessionJoinState`'s dispatcher are registered on this builder.
pub type SessionJoinStateBuilder =
    AppStateBuilder<'static, 'static, SessionJoinStateDelegate, SessionJoinEntity>;

/// Delegate `State` for session joining.
///
/// This state is not intended to be used directly, but wrapped in an `AppState`. The
/// `SessionJoinState` is an alias with this as a delegate state.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct SessionJoinStateDelegate;

impl SessionJoinStateDelegate {
    fn initialize_state(data: StateData<'_, GameData<'static, 'static>>) {
        data.world.insert(StateId::SessionJoin);
    }
}

impl State<GameData<'static, 'static>, AppEvent> for SessionJoinStateDelegate {
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
        if let AppEvent::SessionJoin(session_join_event) = event {
            debug!("Received session_join_event: {:?}", session_join_event);
            match session_join_event {
                SessionJoinEvent::Back => Trans::Pop,
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }
}
