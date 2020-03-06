use std::{
    hash::{Hash, Hasher},
    net::SocketAddr,
};

use derive_new::new;
use serde::{Deserialize, Serialize};

/// Data that came through a network connection.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct NetData<D> {
    /// `SocketAddr` of the sender of the data.
    ///
    /// * If you are reading this on the server, this is the `SocketAddr` of the client.
    /// * If you are reading this on the client, this is the `SocketAddr` of the server.
    pub socket_addr: SocketAddr,
    /// The data.
    pub data: D,
}

impl<D> Hash for NetData<D>
where
    D: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.socket_addr.hash(state);
        self.data.hash(state);
    }
}

impl<D> Eq for NetData<D> where D: Eq {}
