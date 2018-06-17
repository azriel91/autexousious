use object_model::{
    config::object::character::SequenceId,
    entity::{CharacterInput, ObjectStatusUpdate},
};

pub(super) use self::stand::Stand;
pub(super) use self::walk::Walk;

mod stand;
mod walk;

/// Traits that every sequence should define for its transition behaviour.
pub(super) trait SequenceHandler {
    /// Updates behaviour in response to input.
    fn update(input: &CharacterInput) -> ObjectStatusUpdate<SequenceId>;
}
