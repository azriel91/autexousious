use std::collections::HashMap;
use std::hash::Hash;

use config::object::Sequence;

/// Contains all of the sequences for an `Object`.
///
/// This type is not intended to be instantiated by consumers directly. Instead, consumers should
/// instante the various definition types for each object type, such as [`CharacterDefinition`]
/// [char_definition] for characters.
///
/// [char_definition]: ../character/struct.CharacterDefinition.html
#[derive(Clone, Constructor, Debug, Deserialize, PartialEq)]
pub struct ObjectDefinition<SeqId: Eq + Hash> {
    /// Sequences of actions this object can perform.
    pub sequences: HashMap<SeqId, Sequence<SeqId>>,
}
