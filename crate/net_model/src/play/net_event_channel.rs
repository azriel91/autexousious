use amethyst::shrev::EventChannel;

use crate::play::NetData;

/// `EventChannel` for events received over a network connection.
///
/// Type alias for `EventChannel<NetData<E>>`.
pub type NetEventChannel<E> = EventChannel<NetData<E>>;
