use derive_new::new;
use object_model::config::{GameObjectSequence, ObjectSequence};
use serde::{Deserialize, Serialize};

use crate::config::{EnergyFrame, EnergySequenceName};

/// Represents an independent action sequence of an `Energy`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct EnergySequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub object_sequence: ObjectSequence<EnergySequenceName, EnergyFrame>,
}

impl GameObjectSequence for EnergySequence {
    type SequenceName = EnergySequenceName;
    type GameObjectFrame = EnergyFrame;

    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceName, Self::GameObjectFrame> {
        &self.object_sequence
    }
}
