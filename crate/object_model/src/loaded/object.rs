use std::collections::HashMap;

use amethyst::{
    animation::Animation,
    assets::Handle,
    renderer::{SpriteRender, SpriteSheetHandle},
};

use config::object::SequenceId;

/// Represents an in-game object that has been loaded.
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Object<SeqId: SequenceId> {
    /// Handle to the default sprite sheet to use for the character.
    pub default_sprite_sheet: SpriteSheetHandle,
    /// Handle to the sprite animations that this object uses.
    pub animations: HashMap<SeqId, Handle<Animation<SpriteRender>>>,
}
