use std::{collections::HashMap, net::SocketAddr};

use derive_deref::{Deref, DerefMut};
use derive_new::new;
use network_session_model::play::SessionDeviceId;

/// Tracks the `SessionDeviceId` for each `Session`.
///
/// `HashMap<SocketAddr, SessionDeviceId>` newtype.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct SocketToDeviceId(pub HashMap<SocketAddr, SessionDeviceId>);
