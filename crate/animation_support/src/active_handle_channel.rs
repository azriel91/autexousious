use serde::{Deserialize, Serialize};

/// Channels that are animatable on `ActiveHandle`
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ActiveHandleChannel {
    /// The handle to use.
    Handle,
}
