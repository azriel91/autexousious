use amethyst::{GameData, Trans};
use application_event::AppEvent;
use network_mode_selection_model::NetworkModeIndex;
use session_join::{SessionJoinStateBuilder, SessionJoinStateDelegate};

/// Returns the `Trans` for a given `NetworkModeIndex`.
#[derive(Debug)]
pub struct NetworkModeSelectionTrans;

impl NetworkModeSelectionTrans {
    /// Returns a transition when a menu item has been selected.
    ///
    /// # Parameters
    ///
    /// * `network_mode_index`: The selected index.
    pub fn trans(
        network_mode_index: NetworkModeIndex,
    ) -> Trans<GameData<'static, 'static>, AppEvent> {
        match network_mode_index {
            NetworkModeIndex::Host => Trans::None,
            NetworkModeIndex::Join => {
                let state = SessionJoinStateBuilder::new(SessionJoinStateDelegate::new()).build();

                Trans::Push(Box::new(state))
            }
            NetworkModeIndex::Back => Trans::Pop,
        }
    } // kcov-ignore
}
