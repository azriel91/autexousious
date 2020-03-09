use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::loaded::PlayerController;

/// Player names and their controller IDs (`Vec<PlayerController>` newtype).
///
/// This includes local and remote players (if any).
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct PlayerControllers(pub Vec<PlayerController>);
