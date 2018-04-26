use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{Mesh, MeshHandle, PosTex};
use object_model::config::SpritesDefinition;

/// Provides functionality to create meshes used to render an object.
#[derive(Debug)]
pub(super) struct SpriteMeshCreator;

impl SpriteMeshCreator {
    /// Creates a `Mesh` for mapping the object's sprites to screen and returns the `MeshHandle`.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the `Mesh`.
    /// * `sprites_definition`: Sprite sheets layout metadata.
    pub(super) fn create_mesh(world: &World, sprites_definition: &SpritesDefinition) -> MeshHandle {
        let (sprite_w, sprite_h) = {
            sprites_definition
                .sheets
                .first()
                .map_or((1., 1.), |sheet_def| {
                    (sheet_def.sprite_w, sheet_def.sprite_h)
                })
        };

        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            Self::create_mesh_vertices(sprite_w, sprite_h).into(),
            (),
            &world.read_resource::<AssetStorage<Mesh>>(),
        )
    }

    /// Returns a set of vertices that make up a rectangular mesh of the given size.
    ///
    /// This function expects pixel coordinates -- starting from the top left of the image. X
    /// increases to the right, Y increases downwards.
    ///
    /// # Parameters
    ///
    /// * `sprite_w`: Width of each sprite, excluding the border pixel if any.
    /// * `sprite_h`: Height of each sprite, excluding the border pixel if any.
    fn create_mesh_vertices(sprite_w: f32, sprite_h: f32) -> Vec<PosTex> {
        // It's important that the texture coordinates use 0.0 and 1.0, as the `MaterialAnimation`
        // texture offsets are multiplied against these values, instead of substituted in place of
        // them.
        vec![
            PosTex {
                position: [0., 0., 0.],
                tex_coord: [0., 0.],
            },
            PosTex {
                position: [sprite_w, 0., 0.],
                tex_coord: [1., 0.],
            },
            PosTex {
                position: [0., sprite_h, 0.],
                tex_coord: [0., 1.],
            },
            PosTex {
                position: [sprite_w, sprite_h, 0.],
                tex_coord: [1., 1.],
            },
            PosTex {
                position: [0., sprite_h, 0.],
                tex_coord: [0., 1.],
            },
            PosTex {
                position: [sprite_w, 0., 0.],
                tex_coord: [1., 0.],
            },
        ]
    }
}
