use derive_new::new;
use object_model::config::{GameObjectSequence, ObjectSequence};
use sequence_model::config::Sequence;
use serde::{Deserialize, Serialize};

use crate::config::{EnergyFrame, EnergySequenceName};

/// Represents an independent action sequence of an `Energy`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
// #[serde(deny_unknown_fields)] // See <https://github.com/serde-rs/serde/issues/1547>
pub struct EnergySequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub object_sequence: ObjectSequence<EnergySequenceName, EnergyFrame>,
}

impl AsRef<Sequence<EnergySequenceName, EnergyFrame>> for EnergySequence {
    fn as_ref(&self) -> &Sequence<EnergySequenceName, EnergyFrame> {
        &self.object_sequence.sequence
    }
}

impl GameObjectSequence for EnergySequence {
    type GameObjectFrame = EnergyFrame;
    type SequenceName = EnergySequenceName;

    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceName, Self::GameObjectFrame> {
        &self.object_sequence
    }
}
