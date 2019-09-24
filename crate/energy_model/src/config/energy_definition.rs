use asset_derive::Asset;
use derive_new::new;
use object_model::config::ObjectDefinition;
use serde::{Deserialize, Serialize};

use crate::config::EnergySequence;

/// Contains all of the sequences for an `Energy`.
#[derive(Asset, Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct EnergyDefinition {
    /// Sequences of actions this object can perform.
    #[serde(flatten)]
    pub object_definition: ObjectDefinition<EnergySequence>,
}
