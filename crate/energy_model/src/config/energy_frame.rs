#![allow(clippy::nonstandard_macro_braces)] // TODO: Pending https://github.com/rust-lang/rust-clippy/issues/7434

use derive_new::new;
use object_model::config::{GameObjectFrame, ObjectFrame};
use sequence_model::config::Wait;
use serde::{Deserialize, Serialize};

/// Sequence frame type for energies.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct EnergyFrame {
    /// Common object behaviour specification that can change each tick.
    #[serde(flatten)]
    pub object_frame: ObjectFrame,
}

impl AsRef<Wait> for EnergyFrame {
    fn as_ref(&self) -> &Wait {
        &self.object_frame.wait
    }
}

impl GameObjectFrame for EnergyFrame {
    fn object_frame(&self) -> &ObjectFrame {
        &self.object_frame
    }
}
