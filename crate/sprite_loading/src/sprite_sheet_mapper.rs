use amethyst::{
    assets::Handle,
    renderer::{sprite::SpriteSheet, Texture},
};
use asset_gfx_gen::{SpriteGenParams, SpriteSheetGen};
use log::trace;
use sprite_model::config::SpriteSheetDefinition;

/// Maps sprite sheet definitions and texture handles to sprite sheets.
#[derive(Debug)]
pub struct SpriteSheetMapper;

impl SpriteSheetMapper {
    /// Returns Amethyst `SpriteSheet`s mapped from `SpriteSheetDefinition`s.
    ///
    /// # Parameters
    ///
    /// * `texture_handles`: Handles of the sprite sheets' textures.
    /// * `sprite_sheet_definitions`: List of metadata for sprite sheets to map.
    pub fn map(
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

                // Offsets are shifted up by half the sprite width and height, as Amethyst uses
                // the middle of sprites as the pivot point.
                let offset_x = offset_w * col as u32;
                let offset_y = offset_h * row as u32;
                let half_sprite_w = definition.sprite_w as f32 / 2.;
                let half_sprite_h = definition.sprite_h as f32 / 2.;

                let offsets = sprite_offsets.map_or_else(
                    || [-half_sprite_w, -half_sprite_h],
                    |sprite_offsets| {
                        let sprite_index = (row * definition.column_count + col) as usize;
                        let sprite_offset = &sprite_offsets[sprite_index];

                        // Negate the Y value because we want to shift the sprite up, whereas the
                        // offset in Amethyst is to shift it down.
                        //
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
                        // Finally, because Amethyst normally shifts the middle of the sprite to the
                        // XYZ position of the entity, we unshift it.
                        let pixel_offset_x =
                            (sprite_offset.x - offset_x as i32) as f32 - half_sprite_w;
                        let pixel_offset_y =
                            ((offset_h + offset_y) as i32 - sprite_offset.y) as f32 - half_sprite_h;

                        [pixel_offset_x, pixel_offset_y]
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
    /// This is simply the sprite width and height if there is no border between
    /// sprites, or 1 added to the width and height if there is a border.
    /// There is no leading border on the left or top of the leftmost and
    /// topmost sprites.
    ///
    /// Be careful not to confuse this with the sprite offsets on the
    /// definition, which are the offsets used to position the sprite
    /// relative to the entity in game.
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
