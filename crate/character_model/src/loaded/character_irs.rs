use sequence_model_derive::frame_component_data;

use crate::loaded::CharacterInputReactionsHandle;

/// Sequence of input reactions.
///
/// IRS is short for `InputReactionsSequence`.
#[frame_component_data(CharacterInputReactionsHandle)]
pub struct CharacterIrs;
