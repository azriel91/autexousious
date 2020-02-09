use amethyst::{GameData, State, StateData, Trans};
use application_event::AppEvent;
use application_state::{AppState, AppStateBuilder};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use network_join_model::NetworkJoinEntity;
use state_registry::StateId;

/// `State` where game mode selection takes place.
///
/// This state is not intended to be constructed directly, but through the
/// [`NetworkJoinStateBuilder`][state_builder].
///
/// [state_builder]: network_mode_selection_state/struct.NetworkJoinStateBuilder.html
pub type NetworkJoinState = AppState<'static, 'static, NetworkJoinStateDelegate, NetworkJoinEntity>;

/// Builder for a `NetworkJoinState`.
///
/// `SystemBundle`s to run in the `NetworkJoinState`'s dispatcher are registered on this
/// builder.
pub type NetworkJoinStateBuilder =
    AppStateBuilder<'static, 'static, NetworkJoinStateDelegate, NetworkJoinEntity>;

/// Delegate `State` for game mode selection.
///
/// This state is not intended to be used directly, but wrapped in an `AppState`. The
/// `NetworkJoinState` is an alias with this as a delegate state.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct NetworkJoinStateDelegate;

impl NetworkJoinStateDelegate {
    fn initialize_state(data: StateData<'_, GameData<'static, 'static>>) {
        data.world.insert(StateId::NetworkJoin);
    }
}

impl State<GameData<'static, 'static>, AppEvent> for NetworkJoinStateDelegate {
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
        if let AppEvent::NetworkJoin(network_join_event) = event {
            debug!("Received network_join_event: {:?}", network_join_event);
            Trans::None
        } else {
            Trans::None
        }
    }
}
