use std::collections::HashMap;

use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::model::{SessionCodeId, SessionDeviceTickStatuses};

/// Tracks the `SessionDeviceTickStatuses` for each `Session`.
///
/// `HashMap<SessionCodeId, SessionDeviceTickStatuses>` newtype.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct SessionTickStatuses(pub HashMap<SessionCodeId, SessionDeviceTickStatuses>);
