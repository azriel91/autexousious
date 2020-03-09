use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::ControllerId;

/// Player name and their controller ID.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct PlayerController {
    /// Name associated with this controller.
    pub name: String,
    /// In memory controller ID.
    pub controller_id: ControllerId,
}
