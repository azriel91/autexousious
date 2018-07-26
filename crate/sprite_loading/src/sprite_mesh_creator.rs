use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    renderer::{Mesh, MeshHandle, PosTex},
};
use sprite_model::config::SpritesDefinition;

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
        Self::internal_create_mesh(world, sprites_definition, false)
    }

    /// Creates a left-facing `Mesh` for mapping the object's sprites to screen and returns the
    /// `MeshHandle`.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the `Mesh`.
    /// * `sprites_definition`: Sprite sheets layout metadata.
    pub(super) fn create_mesh_mirrored(
        world: &World,
        sprites_definition: &SpritesDefinition,
    ) -> MeshHandle {
        Self::internal_create_mesh(world, sprites_definition, true)
    }

    /// Creates a left-facing `Mesh` for mapping the object's sprites to screen and returns the
    /// `MeshHandle`.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the `Mesh`.
    /// * `sprites_definition`: Sprite sheets layout metadata.
    /// * `mirrored`: Whether the mesh coordinates should be mirrored right to left.
    fn internal_create_mesh(
        world: &World,
        sprites_definition: &SpritesDefinition,
        mirrored: bool,
    ) -> MeshHandle {
        let (sprite_w, sprite_h) = {
            sprites_definition
                .sheets
                .first()
                .map_or((1., 1.), |sheet_def| {
                    (sheet_def.sprite_w as f32, sheet_def.sprite_h as f32)
                })
        };

        let loader = world.read_resource::<Loader>();

        let mesh_vertices = if mirrored {
            Self::create_mesh_vertices_mirrored(sprite_w, sprite_h)
        } else {
            Self::create_mesh_vertices(sprite_w, sprite_h)
        };
        loader.load_from_data(
            mesh_vertices.into(),
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

    /// Returns a set of vertices that make up a rectangular mesh of the given size.
    ///
    /// This function expects pixel coordinates -- starting from the top left of the image. X
    /// increases to the right, Y increases downwards.
    ///
    /// # Parameters
    ///
    /// * `sprite_w`: Width of each sprite, excluding the border pixel if any.
    /// * `sprite_h`: Height of each sprite, excluding the border pixel if any.
    fn create_mesh_vertices_mirrored(sprite_w: f32, sprite_h: f32) -> Vec<PosTex> {
        // It's important that the texture coordinates use 0.0 and 1.0, as the `MaterialAnimation`
        // texture offsets are multiplied against these values, instead of substituted in place of
        // them.
        vec![
            PosTex {
                position: [0., 0., 0.],
                tex_coord: [1., 0.],
            },
            PosTex {
                position: [sprite_w, 0., 0.],
                tex_coord: [0., 0.],
            },
            PosTex {
                position: [0., sprite_h, 0.],
                tex_coord: [1., 1.],
            },
            PosTex {
                position: [sprite_w, sprite_h, 0.],
                tex_coord: [0., 1.],
            },
            PosTex {
                position: [0., sprite_h, 0.],
                tex_coord: [1., 1.],
            },
            PosTex {
                position: [sprite_w, 0., 0.],
                tex_coord: [0., 0.],
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        assets::AssetStorage,
        prelude::*,
        renderer::{Mesh, MeshHandle, PosTex},
    };
    use amethyst_test_support::prelude::*;
    use sprite_model::config::{SpriteOffset, SpriteSheetDefinition, SpritesDefinition};

    use super::SpriteMeshCreator;

    #[test]
    fn loads_created_meshes_into_world() {
        let setup = |world: &mut World| {
            let mesh_handles: [MeshHandle; 2] = [
                SpriteMeshCreator::create_mesh(world, &sprites_definition()),
                SpriteMeshCreator::create_mesh_mirrored(world, &sprites_definition()),
            ];
            world.add_resource(EffectReturn(mesh_handles));
        };
        let assertion = |world: &mut World| {
            let mesh_handles = &world.read_resource::<EffectReturn<[MeshHandle; 2]>>().0;
            let store = world.read_resource::<AssetStorage<Mesh>>();
            assert!(store.get(&mesh_handles[0]).is_some());
            assert!(store.get(&mesh_handles[1]).is_some());
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_created_meshes_into_world", false)
                .with_setup(setup)
                .with_assertion(assertion)
                .run()
                .is_ok()
        )
    }

    #[test]
    fn create_mesh_vertices_generates_correct_coordinates() {
        let vertices = SpriteMeshCreator::create_mesh_vertices(10., 20.);
        assert_eq!(
            vec![
                PosTex {
                    position: [0., 0., 0.],
                    tex_coord: [0., 0.],
                },
                PosTex {
                    position: [10., 0., 0.],
                    tex_coord: [1., 0.],
                },
                PosTex {
                    position: [0., 20., 0.],
                    tex_coord: [0., 1.],
                },
                PosTex {
                    position: [10., 20., 0.],
                    tex_coord: [1., 1.],
                },
                PosTex {
                    position: [0., 20., 0.],
                    tex_coord: [0., 1.],
                },
                PosTex {
                    position: [10., 0., 0.],
                    tex_coord: [1., 0.],
                },
            ],
            vertices
        );
    }

    #[test]
    fn create_mesh_vertices_mirrored_generates_correct_coordinates() {
        let vertices = SpriteMeshCreator::create_mesh_vertices_mirrored(10., 20.);
        assert_eq!(
            vec![
                PosTex {
                    position: [0., 0., 0.],
                    tex_coord: [1., 0.],
                },
                PosTex {
                    position: [10., 0., 0.],
                    tex_coord: [0., 0.],
                },
                PosTex {
                    position: [0., 20., 0.],
                    tex_coord: [1., 1.],
                },
                PosTex {
                    position: [10., 20., 0.],
                    tex_coord: [0., 1.],
                },
                PosTex {
                    position: [0., 20., 0.],
                    tex_coord: [1., 1.],
                },
                PosTex {
                    position: [10., 0., 0.],
                    tex_coord: [0., 0.],
                },
            ],
            vertices
        );
    }

    fn sprites_definition() -> SpritesDefinition {
        SpritesDefinition::new(vec![sprite_sheet_definition()])
    }

    fn sprite_sheet_definition() -> SpriteSheetDefinition {
        SpriteSheetDefinition::new("0.png".to_string(), 9, 19, 2, 3, true, offsets(6))
    }

    fn offsets(n: usize) -> Vec<SpriteOffset> {
        (0..n).map(|_| (0, 0).into()).collect()
    }
}
