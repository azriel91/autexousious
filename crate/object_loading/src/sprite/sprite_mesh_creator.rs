use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    renderer::{Mesh, MeshHandle, PosTex},
};
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
                    (sheet_def.sprite_w, sheet_def.sprite_h)
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
    use std::path::PathBuf;

    use amethyst::{
        assets::AssetStorage,
        prelude::*,
        renderer::{ColorMask, DrawFlat, Mesh, MeshHandle, PosTex, ALPHA},
        Result,
    };
    use application::resource::{
        dir::{self, assets_dir},
        find_in,
    };
    use object_model::config::{SpriteOffset, SpriteSheetDefinition, SpritesDefinition};

    use super::SpriteMeshCreator;

    #[test]
    fn loads_created_meshes_into_world() {
        let setup_fn = |world: &mut World| -> [MeshHandle; 2] {
            [
                SpriteMeshCreator::create_mesh(world, &sprites_definition()),
                SpriteMeshCreator::create_mesh_mirrored(world, &sprites_definition()),
            ] // kcov-ignore
        };
        let assertion_fn = |world: &mut World, mesh_handles: [MeshHandle; 2]| {
            let store = world.read_resource::<AssetStorage<Mesh>>();
            assert!(store.get(&mesh_handles[0]).is_some());
            assert!(store.get(&mesh_handles[1]).is_some());
        };

        assert!(run(Box::new(setup_fn), Box::new(assertion_fn)).is_ok())
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

    fn run<T, F1, F2>(setup_fn: Box<F1>, assertion_fn: Box<F2>) -> Result<()>
    where
        F1: Fn(&mut World) -> T,
        F2: Fn(&mut World, T),
    {
        let assets_dir = assets_dir(Some(development_base_dirs!()))?;
        let test_state = TestState::new(setup_fn, assertion_fn);

        Application::new(assets_dir, test_state, setup_game_data()?)?.run();

        Ok(())
    }

    fn setup_game_data<'a, 'b>() -> Result<GameDataBuilder<'a, 'b>> {
        GameDataBuilder::default().with_basic_renderer(
            display_config()?,
            DrawFlat::<PosTex>::new().with_transparency(ColorMask::all(), ALPHA, None),
            false,
        )
    }

    fn display_config() -> Result<PathBuf> {
        Ok(find_in(
            dir::RESOURCES,
            "display_config.ron",
            Some(development_base_dirs!()),
        )?)
    }

    fn sprites_definition() -> SpritesDefinition {
        SpritesDefinition::new(vec![sprite_sheet_definition()])
    }

    fn sprite_sheet_definition() -> SpriteSheetDefinition {
        SpriteSheetDefinition::new("0.png".to_string(), 9., 19., 2, 3, true, offsets(6))
    }

    fn offsets(n: usize) -> Vec<SpriteOffset> {
        (0..n).map(|_| (0, 0).into()).collect()
    }

    #[derive(Debug)]
    struct TestState<T, F1, F2>
    where
        F1: Fn(&mut World) -> T,
        F2: Fn(&mut World, T),
    {
        first_run: bool,
        setup_output: Option<T>,
        setup_fn: Box<F1>,
        assertion_fn: Box<F2>,
    }
    impl<T, F1, F2> TestState<T, F1, F2>
    where
        F1: Fn(&mut World) -> T,
        F2: Fn(&mut World, T),
    {
        fn new(setup_fn: Box<F1>, assertion_fn: Box<F2>) -> Self {
            TestState {
                first_run: true,
                setup_output: None,
                setup_fn,
                assertion_fn,
            }
        }
    }
    impl<'a, 'b, T, F1, F2> State<GameData<'a, 'b>> for TestState<T, F1, F2>
    where
        F1: Fn(&mut World) -> T,
        F2: Fn(&mut World, T),
    {
        fn update(&mut self, mut data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
            data.data.update(&data.world);

            if self.first_run {
                self.first_run = false;
                self.setup_output = Some((self.setup_fn)(&mut data.world));
                Trans::None
            } else {
                (self.assertion_fn)(
                    &mut data.world,
                    self.setup_output
                        .take()
                        .expect("Expected setup_output to be populated."),
                );
                Trans::Quit
            }
        }
    }
}
