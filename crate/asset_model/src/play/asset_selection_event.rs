use game_input_model::ControllerId;

use crate::play::AssetSelection;

/// Event signalling a change in asset selection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssetSelectionEvent {
    /// Signal to return from `AssetSelectionState`.
    Return,
    /// Player has joined / become active.
    Join {
        /// ID of the controller.
        controller_id: ControllerId,
    },
    /// Player has left / become inactive.
    Leave {
        /// ID of the controller.
        controller_id: ControllerId,
    },
    /// `AssetSelection` has switched.
    Switch {
        /// ID of the controller.
        controller_id: ControllerId,
        /// ID of the selected asset.
        asset_selection: AssetSelection,
    },
    /// `AssetSelection` is confirmed.
    Select {
        /// ID of the controller.
        controller_id: ControllerId,
        /// ID of the selected asset.
        asset_selection: AssetSelection,
    },
    /// Asset has been deselected.
    Deselect {
        /// ID of the controller.
        controller_id: ControllerId,
    },
    /// Confirm `AssetSelection`s.
    Confirm,
}
