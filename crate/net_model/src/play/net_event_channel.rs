use amethyst::shrev::EventChannel;

use crate::play::NetEvent;

/// `EventChannel` for events received over a network connection.
///
/// Type alias for `EventChannel<NetEvent<E>>`.
pub type NetEventChannel<E> = EventChannel<NetEvent<E>>;
