use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterInput, CharacterStatus, ObjectStatusUpdate},
};

pub(super) use self::stand::Stand;
pub(super) use self::walk::Walk;

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
    fn update(
        input: &CharacterInput,
        character_status: &mut CharacterStatus,
    ) -> ObjectStatusUpdate<CharacterSequenceId>;
}
