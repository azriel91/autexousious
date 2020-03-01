use std::net::SocketAddr;

use derive_new::new;
use serde::{Deserialize, Serialize};

/// Event that came through a network connection.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct NetEvent<E> {
    /// `SocketAddr` of the sender of the event.
    ///
    /// * If you are reading this on the server, this is the `SocketAddr` of the client.
    /// * If you are reading this on the client, this is the `SocketAddr` of the server.
    pub socket_addr: SocketAddr,
    /// The event.
    pub event: E,
}
