use amethyst::renderer::{Material, MeshHandle};

/// Material and mesh rendering information for an entity whose sprites have been loaded.
#[derive(Clone, Derivative, new)]
#[derivative(Debug)]
pub struct SpriteMaterialMesh {
    /// Default material for entities that are rendered using sprites.
    ///
    /// Even though practically entities will be displayed with a certain animation at all times,
    /// Amethyst requires us to set a default material for entities. If we don't then it panics.
    #[derivative(Debug = "ignore")]
    pub default_material: Material,
    /// Handle to the mesh to map the sprite texture to the screen.
    pub mesh: MeshHandle,
    /// Handle to the left-facing mesh to map the sprite texture to the screen.
    pub mesh_mirrored: MeshHandle,
}
