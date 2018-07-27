use amethyst::renderer::{Sprite, SpriteSheet};
use sprite_model::config::SpriteSheetDefinition;

#[derive(Debug)]
pub(crate) struct SpriteSheetMapper;

impl SpriteSheetMapper {
    /// Maps `SpriteSheetDefinition`s into Amethyst `SpriteSheet`s and returns their handles.
    ///
    /// # Parameters
    ///
    /// * `texture_index_offset`: Index offset for sprite sheet IDs.
    /// * `sprite_sheet_definitions`: List of metadata for sprite sheets to map.
    pub(crate) fn map(
        texture_index_offset: u64,
        sprite_sheet_definitions: &[SpriteSheetDefinition],
    ) -> Vec<SpriteSheet> {
        sprite_sheet_definitions
            .iter()
            .enumerate()
            .map(|(idx, definition)| {
                Self::definition_to_sprite_sheet(texture_index_offset + idx as u64, definition)
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

        // Push the rows in reverse order because the texture coordinates are treated as beginning
        // from the bottom of the image, whereas for this example I want the top left sprite to be
        // the first sprite
        for row in (0..definition.row_count).rev() {
            for col in 0..definition.column_count {
                // Sprites are numbered in the following pattern:
                //
                //  0  1  2  3  4
                //  5  6  7  8  9
                // 10 11 12 13 14
                // 15 16 17 18 19

                let offset_x = offset_w * col as u32;
                let offset_y = offset_h * row as u32;
                let sprite = Self::create_sprite(
                    image_w as f32,
                    image_h as f32,
                    offset_x as f32,
                    offset_y as f32,
                    (offset_x + definition.sprite_w) as f32,
                    (offset_y + definition.sprite_h) as f32,
                    definition.has_border,
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
    /// * `pixel_left`: Pixel X coordinate of the left side of the sprite.
    /// * `pixel_top`: Pixel Y coordinate of the top of the sprite.
    /// * `pixel_right`: Pixel X coordinate of the right side of the sprite.
    /// * `pixel_bottom`: Pixel Y coordinate of the bottom of the sprite.
    /// * `has_border`: Whether or not there is a 1 pixel border between sprites.
    fn create_sprite(
        image_w: f32,
        image_h: f32,
        pixel_left: f32,
        pixel_top: f32,
        pixel_right: f32,
        pixel_bottom: f32,
        has_border: bool,
    ) -> Sprite {
        // Texture coordinates are expressed as fractions of the position on the image.
        let left = pixel_left / image_w;
        let right = pixel_right / image_w;

        // Need to correct the texture coordinates as the Y axis is flipped.
        let (top, bottom) = if has_border {
            ((pixel_top + 1.) / image_h, (pixel_bottom + 1.) / image_h)
        } else {
            (pixel_top / image_h, pixel_bottom / image_h)
        };

        Sprite {
            left,
            top,
            right,
            bottom,
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
            // Sprites bottom row
            [0., 9. / 30., 21. / 40., 40. / 40.].into(),
            [10. / 30., 19. / 30., 21. / 40., 40. / 40.].into(),
            [20. / 30., 29. / 30., 21. / 40., 40. / 40.].into(),
            // Sprites top row
            [0., 9. / 30., 1. / 40., 20. / 40.].into(),
            [10. / 30., 19. / 30., 1. / 40., 20. / 40.].into(),
            [20. / 30., 29. / 30., 1. / 40., 20. / 40.].into(),
        ];
        let sprite_sheet_0 = SpriteSheet {
            texture_id: 10,
            sprites: sprites_0,
        };
        let sprite_sheet_1 = SpriteSheet {
            texture_id: 11,
            sprites: vec![[0., 19. / 20., 1. / 30., 30. / 30.].into()],
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
            // Sprites bottom row
            [0., 10. / 30., 20. / 40., 40. / 40.].into(),
            [10. / 30., 20. / 30., 20. / 40., 40. / 40.].into(),
            [20. / 30., 30. / 30., 20. / 40., 40. / 40.].into(),
            // Sprites top row
            [0., 10. / 30., 0., 20. / 40.].into(),
            [10. / 30., 20. / 30., 0., 20. / 40.].into(),
            [20. / 30., 30. / 30., 0., 20. / 40.].into(),
        ];
        let sprite_sheet_0 = SpriteSheet {
            texture_id: 10,
            sprites: sprites_0,
        };
        let sprite_sheet_1 = SpriteSheet {
            texture_id: 11,
            sprites: vec![[0., 19. / 20., 1. / 30., 30. / 30.].into()],
        };

        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            vec![sprite_sheet_0, sprite_sheet_1],
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

    fn offsets(n: usize) -> Option<Vec<SpriteOffset>> {
        Some((0..n).map(|_| (0, 0).into()).collect())
    }
}
