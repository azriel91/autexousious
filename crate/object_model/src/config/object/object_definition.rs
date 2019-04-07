use std::collections::HashMap;

use derive_new::new;
use sequence_model::config::SequenceId;
use serde::{Deserialize, Serialize};

use crate::config::object::Sequence;

/// Contains all of the sequences for an `Object`.
///
/// This type is not intended to be instantiated by consumers directly. Instead, consumers should
/// instante the various definition types for each object type, such as [`CharacterDefinition`]
/// [char_definition] for characters.
///
/// [char_definition]: ../character/struct.CharacterDefinition.html
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct ObjectDefinition<SeqId: SequenceId> {
    /// Sequences of actions this object can perform.
    pub sequences: HashMap<SeqId, Sequence<SeqId>>,
}
