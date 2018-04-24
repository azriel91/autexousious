use config::object::ObjectDefinition;
use config::object::character::SequenceId;

/// Contains all of the sequences for an `Object`.
#[derive(Constructor, Debug, Deserialize, PartialEq)]
pub struct CharacterDefinition {
    /// Sequences of actions this object can perform.
    #[serde(flatten)]
    pub object_definition: ObjectDefinition<SequenceId>,
}
