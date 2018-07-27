use std::collections::HashMap;

use amethyst::{animation::Animation, assets::Handle, renderer::Material};
use sprite_model::loaded::SpriteMaterialMesh;

use config::object::SequenceId;

/// Represents an in-game object that has been loaded.
#[derive(Clone, Derivative, new)]
#[derivative(Debug)]
pub struct Object<SeqId: SequenceId> {
    /// Default material for entities of this object.
    pub sprite_material_mesh: SpriteMaterialMesh,
    /// Handle to the animations that this object uses.
    ///
    /// This should be substituted with `loaded::Sequences` which will contain the animations.
    pub animations: HashMap<SeqId, Handle<Animation<Material>>>,
}
