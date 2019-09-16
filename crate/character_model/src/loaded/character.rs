use asset_derive::Asset;
use derive_new::new;
use object_model::game_object;
use sequence_model::loaded::SequenceIdMappings;
use typename_derive::TypeName;

use crate::{
    config::{CharacterDefinition, CharacterSequence, CharacterSequenceName},
    loaded::CharacterCtsHandle,
};

/// Represents an in-game character that has been loaded.
///
/// Each of these fields should be a component that is attached to the character entity.
#[game_object]
#[derive(Asset, Clone, Debug, PartialEq, TypeName, new)]
pub struct Character {
    /// Handles of `ControlTransitions`es sequences that this character uses.
    pub cts_handles: Vec<CharacterCtsHandle>,
    /// Mappings from sequence name to ID, and ID to name.
    pub sequence_id_mappings: SequenceIdMappings<CharacterSequenceName>,
}
