use derivative::Derivative;
use derive_new::new;
use indexmap::IndexMap;
use sequence_model::config::SequenceNameString;
use serde::{Deserialize, Serialize};

use crate::config::GameObjectSequence;

/// Contains all of the sequences for an `Object`.
///
/// This type is not intended to be instantiated by consumers directly. Instead, consumers should
/// instantiate the various definition types for each object type, such as [`CharacterDefinition`]
/// [char_definition] for characters.
///
/// [char_definition]: ../character/struct.CharacterDefinition.html
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize, new)]
#[derivative(Default(bound = ""))] // Don't require `ObjSeq: Default`
pub struct ObjectDefinition<ObjSeq>
where
    ObjSeq: GameObjectSequence,
    ObjSeq::SequenceName: for<'des> Deserialize<'des> + Serialize,
{
    /// Sequences of actions this object can perform.
    pub sequences: IndexMap<SequenceNameString<ObjSeq::SequenceName>, ObjSeq>,
}
