use amethyst::{
    prelude::*,
    renderer::{Material, MaterialDefaults, TextureHandle},
};

#[derive(Debug)]
pub(crate) struct MaterialCreator;

impl MaterialCreator {
    /// Returns a material with the albedo set to the first sprite sheet texture.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` that contains the `MaterialDefaults`.
    /// * `texture_handles`: Texture handles of loaded images.
    pub(crate) fn create_default(world: &World, texture_handles: &[TextureHandle]) -> Material {
        let mat_defaults = world.read_resource::<MaterialDefaults>();
        texture_handles.first().map_or_else(
            || mat_defaults.0.clone(),
            |first_texture| Material {
                albedo: first_texture.clone(),
                ..mat_defaults.0.clone()
            },
        )
    } // kcov-ignore
}
