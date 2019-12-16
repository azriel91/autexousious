use serde::{Deserialize, Serialize};
use strum_macros::EnumDiscriminants;

use crate::MapSelection;

/// Event indicating a map selection.
#[derive(Clone, Copy, Debug, EnumDiscriminants, PartialEq)]
#[strum_discriminants(
    name(MapSelectionEventVariant),
    derive(Deserialize, Serialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub enum MapSelectionEvent {
    /// Signal to return from `MapSelectionState`.
    Return,
    /// Map selection is switched.
    Switch {
        /// ID of the selected map.
        map_selection: MapSelection,
    },
    /// Map has been selected.
    Select {
        /// ID of the selected map.
        map_selection: MapSelection,
    },
    /// Map has been deselected.
    Deselect,
    /// Map selection has been confirmed.
    Confirm,
}
