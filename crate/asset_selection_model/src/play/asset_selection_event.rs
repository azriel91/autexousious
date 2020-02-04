use amethyst::ecs::Entity;
use game_input_model::config::ControllerId;

use crate::play::AssetSelection;

/// Event signalling a change in asset selection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssetSelectionEvent {
    /// Signal to return from the `State`.
    Return,
    /// Player has joined / become active.
    Join {
        /// Entity that the event originated from.
        ///
        /// This may be `None` if sent as a CLI command.
        entity: Option<Entity>,
        /// ID of the controller that sent the event.
        controller_id: ControllerId,
    },
    /// Player has left / become inactive.
    Leave {
        /// Entity that the event originated from.
        ///
        /// This may be `None` if sent as a CLI command.
        entity: Option<Entity>,
        /// ID of the controller that sent the event.
        controller_id: ControllerId,
    },
    /// `AssetSelection` has switched.
    Switch {
        /// Entity that the event originated from.
        ///
        /// This may be `None` if sent as a CLI command.
        entity: Option<Entity>,
        /// ID of the controller that sent the event.
        controller_id: ControllerId,
        /// ID of the selected asset.
        asset_selection: AssetSelection,
    },
    /// `AssetSelection` is confirmed.
    Select {
        /// Entity that the event originated from.
        ///
        /// This may be `None` if sent as a CLI command.
        entity: Option<Entity>,
        /// ID of the controller that sent the event.
        controller_id: ControllerId,
        /// ID of the selected asset.
        asset_selection: AssetSelection,
    },
    /// Asset has been deselected.
    Deselect {
        /// Entity that the event originated from.
        ///
        /// This may be `None` if sent as a CLI command.
        entity: Option<Entity>,
        /// ID of the controller that sent the event.
        controller_id: ControllerId,
    },
    /// Confirm `AssetSelection`s.
    Confirm,
}
