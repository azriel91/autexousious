use std::convert::AsRef;

use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::Wait;

/// Common frame components.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default)]
pub struct Frame {
    /// Number of ticks to wait before the sequence switches to the next frame.
    pub wait: Wait,
}

impl AsRef<Wait> for Frame {
    fn as_ref(&self) -> &Wait {
        &self.wait
    }
}
