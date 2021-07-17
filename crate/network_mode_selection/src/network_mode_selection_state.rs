use amethyst::{GameData, State, StateData, Trans};
use application_event::AppEvent;
use application_state::{AppState, AppStateBuilder};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use menu_model::MenuEvent;
use network_mode_selection_model::NetworkModeSelectionEntity;
use state_registry::StateId;

use crate::NetworkModeSelectionTrans;

/// `State` where network mode selection takes place.
///
/// This state is not intended to be constructed directly, but through the
/// [`NetworkModeSelectionStateBuilder`][state_builder].
///
/// [state_builder]:
/// network_mode_selection_state/struct.NetworkModeSelectionStateBuilder.html
pub type NetworkModeSelectionState =
    AppState<'static, 'static, NetworkModeSelectionStateDelegate, NetworkModeSelectionEntity>;

/// Builder for a `NetworkModeSelectionState`.
///
/// `SystemBundle`s to run in the `NetworkModeSelectionState`'s dispatcher are
/// registered on this builder.
pub type NetworkModeSelectionStateBuilder = AppStateBuilder<
    'static,
    'static,
    NetworkModeSelectionStateDelegate,
    NetworkModeSelectionEntity,
>;

/// Delegate `State` for network mode selection.
///
/// This state is not intended to be used directly, but wrapped in an
/// `AppState`. The `NetworkModeSelectionState` is an alias with this as a
/// delegate state.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct NetworkModeSelectionStateDelegate;

impl NetworkModeSelectionStateDelegate {
    fn initialize_state(data: StateData<'_, GameData<'static, 'static>>) {
        data.world.insert(StateId::NetworkModeSelection);
    }
}

impl State<GameData<'static, 'static>, AppEvent> for NetworkModeSelectionStateDelegate {
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
        if let AppEvent::NetworkModeSelection(network_mode_selection_event) = event {
            debug!(
                "Received network_mode_selection_event: {:?}",
                network_mode_selection_event
            );
            match network_mode_selection_event {
                MenuEvent::Select(idx) => NetworkModeSelectionTrans::trans(idx),
                MenuEvent::Close => Trans::Pop,
            }
        } else {
            Trans::None
        }
    }
}
