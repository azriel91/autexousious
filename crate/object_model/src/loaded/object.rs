use std::collections::HashMap;

use amethyst::renderer::SpriteSheetHandle;

use config::object::SequenceId;
use loaded::AnimatedComponent;

/// Represents an in-game object that has been loaded.
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Object<SeqId: SequenceId> {
    /// Handle to the default sprite sheet to use for the character.
    pub default_sprite_sheet: SpriteSheetHandle,
    /// Handles to the animations that this object uses, keyed by sequence ID.
    pub animations: HashMap<SeqId, Vec<AnimatedComponent>>,
}
