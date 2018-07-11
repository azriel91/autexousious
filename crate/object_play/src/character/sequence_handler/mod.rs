use object_model::entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate};

pub(super) use self::run::Run;
pub(super) use self::stand::Stand;
pub(super) use self::walk::Walk;

mod run;
mod stand;
mod walk;

/// Traits that every sequence should define for its transition behaviour.
pub(super) trait SequenceHandler {
    /// Updates behaviour in response to input.
    ///
    /// # Parameters
    ///
    /// * `input`: Controller input for the character.
    /// * `character_status`: Character specific status attributes.
    fn update(input: &CharacterInput, character_status: &CharacterStatus) -> CharacterStatusUpdate;
}
