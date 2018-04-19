use amethyst::renderer::{Material, SpriteSheetHandle};

/// Represents an in-game object that has been loaded.
#[derive(Constructor, Derivative)]
#[derivative(Debug)]
pub struct Object {
    /// Default material for entities of this object.
    ///
    /// Even though practically entities will be displayed with a certain animation at all times,
    /// Amethyst requires us to set a default material for entities. If we don't then it panics.
    #[derivative(Debug = "ignore")]
    pub default_material: Material,
    /// Handle to the sprite sheets that this object uses.
    ///
    /// This should be replaced with animations when that part is implemented.
    ///
    /// Later on, that will be substituted with `Sequences` which will contain the animations.
    pub sprite_sheets: Vec<SpriteSheetHandle>,
}
