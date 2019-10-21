#[cfg(test)]
mod test {
    use amethyst::{
        assets::{AssetStorage, Handle, Loader},
        core::TransformBundle,
        ecs::{World, WorldExt},
        renderer::{
            loaders::load_from_srgba,
            palette::Srgba,
            types::{DefaultBackend, TextureData},
            RenderEmptyBundle, SpriteSheet, Texture,
        },
        Error,
    };
    use amethyst_test::AmethystApplication;
    use sprite_model::config::{SpriteOffset, SpriteSheetDefinition};

    use sprite_loading::SpriteSheetMapper;

    #[test]
    fn map_multiple_sprite_sheet_definitions() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_assertion(|world| {
                let sprite_sheet_definitions = [sprite_sheet_definition(true), simple_definition()];
                let texture_handles = test_texture_handles(world);

                let sprites_0 = vec![
                    // Sprites top row
                    (
                        (9., 19.),
                        [-4.5, 10.5],
                        [0.5 / 30., 8.5 / 30., 18.5 / 40., 0.5 / 40.],
                    )
                        .into(),
                    (
                        (9., 19.),
                        [-13.5, 9.5],
                        [10.5 / 30., 18.5 / 30., 18.5 / 40., 0.5 / 40.],
                    )
                        .into(),
                    (
                        (9., 19.),
                        [-22.5, 8.5],
                        [20.5 / 30., 28.5 / 30., 18.5 / 40., 0.5 / 40.],
                    )
                        .into(),
                    // Sprites bottom row
                    (
                        (9., 19.),
                        [-1.5, 27.5],
                        [0.5 / 30., 8.5 / 30., 38.5 / 40., 20.5 / 40.],
                    )
                        .into(),
                    (
                        (9., 19.),
                        [-10.5, 26.5],
                        [10.5 / 30., 18.5 / 30., 38.5 / 40., 20.5 / 40.],
                    )
                        .into(),
                    (
                        (9., 19.),
                        [-19.5, 25.5],
                        [20.5 / 30., 28.5 / 30., 38.5 / 40., 20.5 / 40.],
                    )
                        .into(),
                ];
                let sprite_sheet_0 = SpriteSheet {
                    texture: texture_handles[0].clone(),
                    sprites: sprites_0,
                };
                let sprite_sheet_1 = SpriteSheet {
                    texture: texture_handles[1].clone(),
                    sprites: vec![(
                        (19., 29.),
                        [-9.5, 15.5],
                        [0.5 / 20., 18.5 / 20., 28.5 / 30., 0.5 / 30.],
                    )
                        .into()],
                }; // kcov-ignore

                // kcov-ignore-start
                assert_eq!(
                    // kcov-ignore-end
                    vec![sprite_sheet_0, sprite_sheet_1],
                    SpriteSheetMapper::map(&texture_handles, &sprite_sheet_definitions)
                );
            })
            .run()
    }

    #[test]
    fn map_sprite_sheet_definition_without_border() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_assertion(|world| {
                let sprite_sheet_definitions =
                    [sprite_sheet_definition(false), simple_definition()];
                let texture_handles = test_texture_handles(world);

                let sprites_0 = vec![
                    // Sprites top row
                    (
                        (10., 20.),
                        [-5., 10.],
                        [0.5 / 30., 9.5 / 30., 19.5 / 40., 0.5 / 40.],
                    )
                        .into(),
                    (
                        (10., 20.),
                        [-14., 9.],
                        [10.5 / 30., 19.5 / 30., 19.5 / 40., 0.5 / 40.],
                    )
                        .into(),
                    (
                        (10., 20.),
                        [-23., 8.],
                        [20.5 / 30., 29.5 / 30., 19.5 / 40., 0.5 / 40.],
                    )
                        .into(),
                    // Sprites bottom row
                    (
                        (10., 20.),
                        [-2., 27.],
                        [0.5 / 30., 9.5 / 30., 39.5 / 40., 20.5 / 40.],
                    )
                        .into(),
                    (
                        (10., 20.),
                        [-11., 26.],
                        [10.5 / 30., 19.5 / 30., 39.5 / 40., 20.5 / 40.],
                    )
                        .into(),
                    (
                        (10., 20.),
                        [-20., 25.],
                        [20.5 / 30., 29.5 / 30., 39.5 / 40., 20.5 / 40.],
                    )
                        .into(),
                ];
                let sprite_sheet_0 = SpriteSheet {
                    texture: texture_handles[0].clone(),
                    sprites: sprites_0,
                };
                let sprite_sheet_1 = SpriteSheet {
                    texture: texture_handles[1].clone(),
                    sprites: vec![(
                        (19., 29.),
                        [-9.5, 15.5],
                        [0.5 / 20., 18.5 / 20., 28.5 / 30., 0.5 / 30.],
                    )
                        .into()],
                };

                // kcov-ignore-start
                assert_eq!(
                    // kcov-ignore-end
                    vec![sprite_sheet_0, sprite_sheet_1],
                    SpriteSheetMapper::map(&texture_handles, &sprite_sheet_definitions)
                );
            })
            .run()
    }

    #[test]
    fn offsets_defaults_to_negated_half_sprite_dimensions_if_none() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_assertion(|world| {
                let sprite_sheet_definitions = [no_offsets_definition()];
                let texture_handles = test_texture_handles(world);

                let sprite_sheet = SpriteSheet {
                    texture: texture_handles[0].clone(),
                    sprites: vec![(
                        (19., 29.),
                        [-9.5, -14.5],
                        [0.5 / 20., 18.5 / 20., 28.5 / 30., 0.5 / 30.],
                    )
                        .into()],
                }; // kcov-ignore

                // kcov-ignore-start
                assert_eq!(
                    // kcov-ignore-end
                    vec![sprite_sheet],
                    SpriteSheetMapper::map(&texture_handles, &sprite_sheet_definitions)
                );
            })
            .run()
    }

    fn simple_definition() -> SpriteSheetDefinition {
        SpriteSheetDefinition::new(
            String::from("bat_brown.png"),
            19,
            29,
            1,
            1,
            true,
            offsets(1),
        )
    }

    fn sprite_sheet_definition(with_border: bool) -> SpriteSheetDefinition {
        if with_border {
            SpriteSheetDefinition::new(String::from("bat_grey.png"), 9, 19, 2, 3, true, offsets(6))
        } else {
            SpriteSheetDefinition::new(
                String::from("bat_grey.png"),
                10,
                20,
                2,
                3,
                false,
                offsets(6),
            )
        }
    }

    fn no_offsets_definition() -> SpriteSheetDefinition {
        SpriteSheetDefinition::new(String::from("bat_brown.png"), 19, 29, 1, 1, true, None)
    }

    fn offsets(n: usize) -> Option<Vec<SpriteOffset>> {
        Some((0..n).map(|i| (i as i32, i as i32).into()).collect())
    }

    fn test_texture_handles(world: &mut World) -> Vec<Handle<Texture>> {
        vec![test_texture_handle(world), test_texture_handle(world)]
    }

    fn test_texture_handle(world: &mut World) -> Handle<Texture> {
        let loader = world.read_resource::<Loader>();
        let texture_assets = world.read_resource::<AssetStorage<Texture>>();

        let texture_builder = load_from_srgba(Srgba::new(0., 0., 0., 1.));
        let texture_data = TextureData::from(texture_builder);
        loader.load_from_data(texture_data, (), &texture_assets)
    }
}
