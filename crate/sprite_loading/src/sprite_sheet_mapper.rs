use amethyst::renderer::{Sprite, SpriteSheet, TextureCoordinates};
use sprite_model::config::SpriteSheetDefinition;

#[derive(Debug)]
pub(crate) struct SpriteSheetMapper;

impl SpriteSheetMapper {
    /// Returns Amethyst `SpriteSheet`s mapped from `SpriteSheetDefinition`s.
    ///
    /// # Parameters
    ///
    /// * `sprite_sheet_index_offset`: Index offset for sprite sheet IDs.
    /// * `sprite_sheet_definitions`: List of metadata for sprite sheets to map.
    pub(crate) fn map(
        sprite_sheet_index_offset: u64,
        sprite_sheet_definitions: &[SpriteSheetDefinition],
    ) -> Vec<SpriteSheet> {
        sprite_sheet_definitions
            .iter()
            .enumerate()
            .map(|(idx, definition)| {
                Self::definition_to_sprite_sheet(sprite_sheet_index_offset + idx as u64, definition)
            })
            .collect::<Vec<SpriteSheet>>()
    }

    /// Converts a `SpriteSheetDefinition` into a `SpriteSheet`.
    ///
    /// # Parameters:
    ///
    /// * `texture_id`: ID of the sprite sheet's texture in the `MaterialTextureSet`.
    /// * `definition`: Definition of the sprite layout on the sprite sheet.
    fn definition_to_sprite_sheet(
        texture_id: u64,
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

                let offset_x = offset_w * col as u32;
                let offset_y = offset_h * row as u32;
                let offsets = sprite_offsets.map_or_else(
                    || [0.; 2],
                    |sprite_offsets| {
                        let sprite_index = (row * definition.column_count + col) as usize;
                        let sprite_offset = &sprite_offsets[sprite_index];

                        [
                            (sprite_offset.x - offset_x as i32) as f32,
                            // Negate the Y value because we want to shift the sprite up, whereas
                            // the offset in Amethyst is to shift it down.
                            (offset_y as i32 - sprite_offset.y) as f32,
                        ]
                    },
                );
                let sprite = Self::create_sprite(
                    image_w as f32,
                    image_h as f32,
                    definition.sprite_w as f32,
                    definition.sprite_h as f32,
                    offset_x as f32,
                    offset_y as f32,
                    offsets,
                );

                let sprite_number = row * definition.column_count + col;
                trace!("{}: Sprite: {:?}", sprite_number, &sprite);

                sprites.push(sprite);
            }
        }

        SpriteSheet {
            texture_id,
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

    /// Returns a set of vertices that make up a rectangular mesh of the given size.
    ///
    /// This function expects pixel coordinates -- starting from the top left of the image. X
    /// increases to the right, Y increases downwards.
    ///
    /// # Parameters
    ///
    /// * `image_w`: Width of the full sprite sheet.
    /// * `image_h`: Height of the full sprite sheet.
    /// * `sprite_w`: Width of the sprite.
    /// * `sprite_h`: Height of the sprite.
    /// * `pixel_left`: Pixel X coordinate of the left side of the sprite.
    /// * `pixel_top`: Pixel Y coordinate of the top of the sprite.
    fn create_sprite(
        image_w: f32,
        image_h: f32,
        sprite_w: f32,
        sprite_h: f32,
        pixel_left: f32,
        pixel_top: f32,
        offsets: [f32; 2],
    ) -> Sprite {
        let pixel_right = pixel_left + sprite_w;
        let pixel_bottom = pixel_top + sprite_h;

        // Texture coordinates are expressed as fractions of the position on the image.
        // Y axis texture coordinates start at the bottom of the image, so we have to invert them.
        //
        // The 0.5 offsets is to get pixel perfection. See
        // <http://www.mindcontrol.org/~hplus/graphics/opengl-pixel-perfect.html>
        let left = (pixel_left + 0.5) / image_w;
        let right = (pixel_right - 0.5) / image_w;
        let top = (image_h - (pixel_top + 0.5)) / image_h;
        let bottom = (image_h - (pixel_bottom - 0.5)) / image_h;

        let tex_coords = TextureCoordinates {
            left,
            right,
            top,
            bottom,
        };

        Sprite {
            width: sprite_w,
            height: sprite_h,
            offsets,
            tex_coords,
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::renderer::SpriteSheet;
    use sprite_model::config::{SpriteOffset, SpriteSheetDefinition};

    use super::SpriteSheetMapper;

    #[test]
    fn map_multiple_sprite_sheet_definitions() {
        let sprite_sheet_definitions = [sprite_sheet_definition(true), simple_definition()];

        let sprites_0 = vec![
            // Sprites top row
            (
                (9., 19.),
                [0., 0.],
                [0.5 / 30., 8.5 / 30., 21.5 / 40., 39.5 / 40.],
            )
                .into(),
            (
                (9., 19.),
                [-9., -1.],
                [10.5 / 30., 18.5 / 30., 21.5 / 40., 39.5 / 40.],
            )
                .into(),
            (
                (9., 19.),
                [-18., -2.],
                [20.5 / 30., 28.5 / 30., 21.5 / 40., 39.5 / 40.],
            )
                .into(),
            // Sprites bottom row
            (
                (9., 19.),
                [3., 17.],
                [0.5 / 30., 8.5 / 30., 1.5 / 40., 19.5 / 40.],
            )
                .into(),
            (
                (9., 19.),
                [-6., 16.],
                [10.5 / 30., 18.5 / 30., 1.5 / 40., 19.5 / 40.],
            )
                .into(),
            (
                (9., 19.),
                [-15., 15.],
                [20.5 / 30., 28.5 / 30., 1.5 / 40., 19.5 / 40.],
            )
                .into(),
        ];
        let sprite_sheet_0 = SpriteSheet {
            texture_id: 10,
            sprites: sprites_0,
        };
        let sprite_sheet_1 = SpriteSheet {
            texture_id: 11,
            sprites: vec![(
                (19., 29.),
                [0., 0.],
                [0.5 / 20., 18.5 / 20., 1.5 / 30., 29.5 / 30.],
            )
                .into()],
        };

        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            vec![sprite_sheet_0, sprite_sheet_1],
            SpriteSheetMapper::map(10, &sprite_sheet_definitions)
        );
    }

    #[test]
    fn map_sprite_sheet_definition_without_border() {
        let sprite_sheet_definitions = [sprite_sheet_definition(false), simple_definition()];

        let sprites_0 = vec![
            // Sprites top row
            (
                (10., 20.),
                [0., 0.],
                [0.5 / 30., 9.5 / 30., 20.5 / 40., 39.5 / 40.],
            )
                .into(),
            (
                (10., 20.),
                [-9., -1.],
                [10.5 / 30., 19.5 / 30., 20.5 / 40., 39.5 / 40.],
            )
                .into(),
            (
                (10., 20.),
                [-18., -2.],
                [20.5 / 30., 29.5 / 30., 20.5 / 40., 39.5 / 40.],
            )
                .into(),
            // Sprites bottom row
            (
                (10., 20.),
                [3., 17.],
                [0.5 / 30., 9.5 / 30., 0.5 / 40., 19.5 / 40.],
            )
                .into(),
            (
                (10., 20.),
                [-6., 16.],
                [10.5 / 30., 19.5 / 30., 0.5 / 40., 19.5 / 40.],
            )
                .into(),
            (
                (10., 20.),
                [-15., 15.],
                [20.5 / 30., 29.5 / 30., 0.5 / 40., 19.5 / 40.],
            )
                .into(),
        ];
        let sprite_sheet_0 = SpriteSheet {
            texture_id: 10,
            sprites: sprites_0,
        };
        let sprite_sheet_1 = SpriteSheet {
            texture_id: 11,
            sprites: vec![(
                (19., 29.),
                [0., 0.],
                [0.5 / 20., 18.5 / 20., 1.5 / 30., 29.5 / 30.],
            )
                .into()],
        };

        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            vec![sprite_sheet_0, sprite_sheet_1],
            SpriteSheetMapper::map(10, &sprite_sheet_definitions)
        );
    }

    #[test]
    fn offsets_defaults_to_zero_if_none() {
        let sprite_sheet_definitions = [no_offsets_definition()];

        let sprite_sheet = SpriteSheet {
            texture_id: 10,
            sprites: vec![(
                (19., 29.),
                [0., 0.],
                [0.5 / 20., 18.5 / 20., 1.5 / 30., 29.5 / 30.],
            )
                .into()],
        };

        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            vec![sprite_sheet],
            SpriteSheetMapper::map(10, &sprite_sheet_definitions)
        );
    }

    fn simple_definition() -> SpriteSheetDefinition {
        SpriteSheetDefinition::new("1.png".to_string(), 19, 29, 1, 1, true, offsets(1))
    }

    fn sprite_sheet_definition(with_border: bool) -> SpriteSheetDefinition {
        if with_border {
            SpriteSheetDefinition::new("0.png".to_string(), 9, 19, 2, 3, true, offsets(6))
        } else {
            SpriteSheetDefinition::new("0.png".to_string(), 10, 20, 2, 3, false, offsets(6))
        }
    }

    fn no_offsets_definition() -> SpriteSheetDefinition {
        SpriteSheetDefinition::new("1.png".to_string(), 19, 29, 1, 1, true, None)
    }

    fn offsets(n: usize) -> Option<Vec<SpriteOffset>> {
        Some((0..n).map(|i| (i as i32, i as i32).into()).collect())
    }
}
