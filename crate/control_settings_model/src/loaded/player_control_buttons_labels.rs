use derive_new::new;
use game_input_model::{config::ControlAction, loaded::ControlAxis};
use indexmap::IndexMap;

use crate::config::ControlButtonLabel;

/// `ControlButtonLabels` for each player.
///
/// These are useful to display control button keys on screen.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct PlayerControlButtonsLabels {
    /// Axis control button labels.
    pub axes: IndexMap<ControlAxis, ControlButtonLabel>,
    /// Action control button labels.
    pub actions: IndexMap<ControlAction, ControlButtonLabel>,
}
