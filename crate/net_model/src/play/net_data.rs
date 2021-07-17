use std::net::SocketAddr;

use derive_new::new;
use serde::{Deserialize, Serialize};

/// Data that came through a network connection.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
pub struct NetData<D> {
    /// `SocketAddr` of the sender of the data.
    ///
    /// * If you are reading this on the server, this is the `SocketAddr` of the
    ///   client.
    /// * If you are reading this on the client, this is the `SocketAddr` of the
    ///   server.
    pub socket_addr: SocketAddr,
    /// The data.
    pub data: D,
}
