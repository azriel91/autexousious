use amethyst::ecs::prelude::*;
use game_input::{ControllerInput, InputControlled};
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, ObjectStatus, RunCounter},
    loaded::{CharacterHandle, ObjectHandle},
};

/// Character specific `Component` storages.
///
/// These are the storages for the components specific to character objects. See also
/// `ObjectComponentStorages`.
pub type CharacterComponentStorages<'s> = (
    WriteStorage<'s, InputControlled>,
    WriteStorage<'s, ControllerInput>,
    WriteStorage<'s, CharacterHandle>,
    WriteStorage<'s, ObjectHandle<CharacterSequenceId>>,
    WriteStorage<'s, CharacterStatus>,
    WriteStorage<'s, ObjectStatus<CharacterSequenceId>>,
    WriteStorage<'s, RunCounter>,
);
