use std::collections::HashMap;

use asset_derive::Asset;
use derive_new::new;
use object_model::game_object;
use typename_derive::TypeName;

use crate::{
    config::{CharacterDefinition, CharacterSequence, CharacterSequenceId},
    loaded::CharacterControlTransitionsSequenceHandle,
};

/// Represents an in-game character that has been loaded.
///
/// Each of these fields should be a component that is attached to the character entity.
#[game_object(CharacterSequenceId)]
#[derive(Asset, Clone, Debug, PartialEq, TypeName, new)]
pub struct Character {
    /// Handles of `ControlTransitions`es sequences that this character uses, keyed by sequence ID.
    pub control_transitions_sequence_handles:
        HashMap<CharacterSequenceId, CharacterControlTransitionsSequenceHandle>,
}
