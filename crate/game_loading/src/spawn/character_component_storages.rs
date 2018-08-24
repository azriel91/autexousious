use amethyst::ecs::prelude::*;
use game_input::{ControllerInput, InputControlled};
use object_model::{entity::CharacterStatus, loaded::CharacterHandle};

/// Character specific `Component` storages.
///
/// These are the storages for the components specific to character objects. See also
/// `ObjectComponentStorages`.
pub type CharacterComponentStorages<'s> = (
    WriteStorage<'s, InputControlled>,
    WriteStorage<'s, ControllerInput>,
    WriteStorage<'s, CharacterHandle>,
    WriteStorage<'s, CharacterStatus>,
);
