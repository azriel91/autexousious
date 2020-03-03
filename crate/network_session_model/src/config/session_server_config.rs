use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};

/// Configuration needed to connect to the session server.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SessionServerConfig {
    /// Address of the server.
    ///
    /// Currently must be an `IpAddr`, in the future we may accept hostnames.
    pub address: IpAddr,
    /// Port that the server is listening on.
    pub port: u16,
}

impl Default for SessionServerConfig {
    fn default() -> Self {
        SessionServerConfig {
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 1234,
        }
    }
}
