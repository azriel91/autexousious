use std::collections::HashMap;

use derive_deref::{Deref, DerefMut};
use derive_new::new;
use network_session_model::play::SessionDeviceId;

use crate::model::GameInputTickStatus;

/// Tracks which `SessionDeviceId`s have sent the `GameInputTick` message.
///
/// `HashMap<SessionDeviceId, GameInputTickStatus>` newtype.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct SessionDeviceTickStatuses(pub HashMap<SessionDeviceId, GameInputTickStatus>);
