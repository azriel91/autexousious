use amethyst::{
    assets::Handle,
    renderer::{sprite::SpriteSheet, Texture},
};
use asset_gfx_gen::{SpriteGenParams, SpriteSheetGen};
use log::trace;
use sprite_model::config::SpriteSheetDefinition;

#[derive(Debug)]
pub(crate) struct SpriteSheetMapper;

impl SpriteSheetMapper {
    /// Returns Amethyst `SpriteSheet`s mapped from `SpriteSheetDefinition`s.
    ///
    /// # Parameters
    ///
    /// * `texture_handles`: Handles of the sprite sheets' textures.
    /// * `sprite_sheet_definitions`: List of metadata for sprite sheets to map.
    pub(crate) fn map(
        texture_handles: &[Handle<Texture>],
        sprite_sheet_definitions: &[SpriteSheetDefinition],
    ) -> Vec<SpriteSheet> {
        sprite_sheet_definitions
            .iter()
            .enumerate()
            .map(|(index, definition)| {
                Self::definition_to_sprite_sheet(texture_handles[index].clone(), definition)
            })
            .collect::<Vec<SpriteSheet>>()
    }

    /// Converts a `SpriteSheetDefinition` into a `SpriteSheet`.
    ///
    /// # Parameters:
    ///
    /// * `texture_handle`: Handle of the sprite sheet's texture.
    /// * `definition`: Definition of the sprite layout on the sprite sheet.
    fn definition_to_sprite_sheet(
        texture_handle: Handle<Texture>,
        definition: &SpriteSheetDefinition,
    ) -> SpriteSheet {
        let mut sprites =
            Vec::with_capacity((definition.row_count * definition.column_count) as usize);
        let (offset_w, offset_h) = Self::offset_distances(definition);
        let (image_w, image_h) = (
            offset_w * definition.column_count,
            offset_h * definition.row_count,
        );

        let sprite_offsets = definition.offsets.as_ref();

        for row in 0..definition.row_count {
            for col in 0..definition.column_count {
                // Sprites are numbered in the following pattern:
                //
                //  0  1  2  3  4
                //  5  6  7  8  9
                // 10 11 12 13 14
                // 15 16 17 18 19

                // Offsets are shifted up by half the sprite width and height, as Amethyst uses the
                // middle of sprites as the pivot point.
                let offset_x = offset_w * col as u32;
                let offset_y = offset_h * row as u32;
                let half_sprite_w = definition.sprite_w as f32 / 2.;
                let half_sprite_h = definition.sprite_h as f32 / 2.;

                let offsets = sprite_offsets.map_or_else(
                    || [-half_sprite_w, -half_sprite_h],
                    |sprite_offsets| {
                        let sprite_index = (row * definition.column_count + col) as usize;
                        let sprite_offset = &sprite_offsets[sprite_index];

                        [
                            (sprite_offset.x - offset_x as i32) as f32 - half_sprite_w,
                            // Negate the Y value because we want to shift the sprite up, whereas
                            // the offset in Amethyst is to shift it down.

                            // * The Y offset specified by the designer should be the bottom of the
                            //   sprite in pixel coordinates for the whole image.
                            // * `offset_y` is the top of the sprite in pixel coordinates.
                            // * Amethyst renders from bottom up (Y axis increases upwards).
                            //
                            // The number of pixels to be below the character's XYZ position is
                            // calculated by the pixel coordinate of the bottom of the sprite,
                            // subtracting the offset specified by the designer (which is usually
                            // within the bounds of the sprite).
                            //
                            // Finally, because Amethyst normally shifts the middle of the sprite to
                            // the XYZ position of the entity, we unshift it.
                            ((offset_h + offset_y) as i32 - sprite_offset.y) as f32 - half_sprite_h,
                        ]
                    },
                );

                let sprite_gen_params = SpriteGenParams {
                    image_w,
                    image_h,
                    sprite_w: definition.sprite_w,
                    sprite_h: definition.sprite_h,
                    pixel_left: offset_x,
                    pixel_top: offset_y,
                    offsets,
                };

                let sprite = SpriteSheetGen::HalfPixel.sprite_from_pixel_values(sprite_gen_params);

                let sprite_number = row * definition.column_count + col;
                trace!("{}: Sprite: {:?}", sprite_number, &sprite);

                sprites.push(sprite);
            }
        }

        SpriteSheet {
            texture: texture_handle,
            sprites,
        }
    }

    /// Returns the pixel offset distances per sprite.
    ///
    /// This is simply the sprite width and height if there is no border between sprites, or 1 added
    /// to the width and height if there is a border. There is no leading border on the left or top
    /// of the leftmost and topmost sprites.
    ///
    /// Be careful not to confuse this with the sprite offsets on the definition, which are the
    /// offsets used to position the sprite relative to the entity in game.
    ///
    /// # Parameters
    ///
    /// * `definition`: Sprite sheet definition.
    fn offset_distances(definition: &SpriteSheetDefinition) -> (u32, u32) {
        if definition.has_border {
            (definition.sprite_w + 1, definition.sprite_h + 1)
        } else {
            (definition.sprite_w, definition.sprite_h)
        }
    }
}

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

    use super::SpriteSheetMapper;

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
            .run_isolated()
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
            .run_isolated()
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
            .run_isolated()
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
