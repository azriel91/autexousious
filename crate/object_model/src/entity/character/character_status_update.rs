use config::object::CharacterSequenceId;
use entity::{ObjectStatusUpdate, RunCounter};

/// Indicates what fields of a `CharacterStatus` should be updated.
#[derive(Default, Debug, PartialEq, new)]
pub struct CharacterStatusUpdate {
    /// Tracks state used to determine when a character should run.
    pub run_counter: Option<RunCounter>,
    /// Common object status for all object type entities.
    pub object_status: ObjectStatusUpdate<CharacterSequenceId>,
}