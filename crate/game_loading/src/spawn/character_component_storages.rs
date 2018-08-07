use amethyst::ecs::prelude::*;
use character_selection::CharacterEntityControl;
use object_model::{entity::CharacterStatus, loaded::CharacterHandle};

/// Character specific `Component` storages.
///
/// These are the storages for the components specific to character objects. See also
/// `ObjectComponentStorages`.
pub type CharacterComponentStorages<'s> = (
    WriteStorage<'s, CharacterEntityControl>,
    WriteStorage<'s, CharacterHandle>,
    WriteStorage<'s, CharacterStatus>,
);
