use std::mem;

use amethyst::renderer::{Sprite, SpriteRender, SpriteSheet, TextureData, TextureMetadata};

use crate::{ColourSpriteSheetGenData, ColourSpriteSheetParams};

/// Generates solid colour `Texture`s and `SpriteSheet`s.
#[derive(Debug)]
pub struct ColourSpriteSheetGen;

impl ColourSpriteSheetGen {
    /// Returns a `SpriteRender` that represents a single pixel sprite with the given colour.
    ///
    /// # Parameters
    ///
    /// * `colour_sprite_gen_data`: System data needed to load colour sprites.
    /// * `colour`: The colour's RGBA values, each between `0.` and `1.`.
    pub fn solid(
        ColourSpriteSheetGenData {
            loader,
            texture_assets,
            sprite_sheet_assets,
        }: &ColourSpriteSheetGenData,
        colour: [f32; 4],
    ) -> SpriteRender {
        let sprite_sheet_handle = {
            let texture_handle =
                loader.load_from_data(TextureData::from(colour), (), &texture_assets);
            let sprite = Sprite::from_pixel_values(1, 1, 1, 1, 0, 0, [0.; 2]);
            let sprites = vec![sprite];

            let sprite_sheet = SpriteSheet {
                texture: texture_handle,
                sprites,
            };

            loader.load_from_data(sprite_sheet, (), sprite_sheet_assets)
        };

        SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        }
    }

    /// Returns a `SpriteRender` that represents a single pixel sprite with the given colour.
    ///
    /// # Parameters
    ///
    /// * `colour_sprite_gen_data`: System data needed to load colour sprites.
    /// * `colour_begin`: The beginning colour's RGBA values, each between `0.` and `1.`.
    /// * `colour_end`: The ending colour's RGBA values, each between `0.` and `1.`.
    /// * `sprite_count`: Number of discreet colour sprites to generate, minimum 2.
    pub fn gradient(
        ColourSpriteSheetGenData {
            loader,
            texture_assets,
            sprite_sheet_assets,
        }: &ColourSpriteSheetGenData,
        colour_begin: [f32; 4],
        colour_end: [f32; 4],
        sprite_count: usize,
    ) -> SpriteRender {
        if sprite_count < 2 {
            panic!(
                "`sprite_count` must be at least 2, received: `{}`.",
                sprite_count
            );
        }

        let sprite_sheet_handle = {
            let params = ColourSpriteSheetParams {
                sprite_w: 1,
                sprite_h: 1,
                padded: true,
                row_count: 1,
                column_count: sprite_count,
            };

            let (texture_metadata, colours) =
                Self::gradient_colours_generate(colour_begin, colour_end, params);
            let (image_width, image_height) = texture_metadata
                .size
                .as_ref()
                .cloned()
                .expect("Expected `texture_metadata` image size to exist.");

            let sprite_count = params.row_count * params.column_count;
            let mut sprites = Vec::with_capacity(sprite_count);

            let padding_pixels = if params.padded { 1 } else { 0 };
            let offset_w = params.sprite_w + padding_pixels;
            let offset_h = params.sprite_h + padding_pixels;
            for row in 0..params.row_count {
                for col in 0..params.column_count {
                    let offset_x = offset_w * col as u32;
                    let offset_y = offset_h * row as u32;
                    let sprite = Sprite::from_pixel_values(
                        image_width as u32,
                        image_height as u32,
                        params.sprite_w,
                        params.sprite_h,
                        offset_x,
                        offset_y,
                        [0.; 2],
                    );

                    let sprite_index = col * params.row_count + row;
                    sprites[sprite_index] = sprite;
                }
            }

            let texture_data = TextureData::F32(colours, texture_metadata);
            let texture_handle = loader.load_from_data(texture_data, (), &texture_assets);
            let sprite_sheet = SpriteSheet {
                texture: texture_handle,
                sprites,
            };

            loader.load_from_data(sprite_sheet, (), sprite_sheet_assets)
        };

        SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        }
    }

    fn gradient_colours_generate(
        colour_begin: [f32; 4],
        colour_end: [f32; 4],
        ColourSpriteSheetParams {
            sprite_w,
            sprite_h,
            padded,
            row_count,
            column_count,
        }: ColourSpriteSheetParams,
    ) -> (TextureMetadata, Vec<f32>) {
        // Pixel count.
        let padding_pixels = if padded { 1 } else { 0 };
        let image_width = (sprite_w + padding_pixels) as usize * column_count;
        let image_height = (sprite_h + padding_pixels) as usize * row_count;
        let pixel_count = image_width * image_height;

        // Byte count.
        let pixel_width = mem::size_of::<f32>();
        let capacity = pixel_count * pixel_width;
        let mut pixel_data = Vec::with_capacity(capacity);

        // Calculate colour values.
        //
        // Pixel coordinates are used, so Y increases downwards.

        let channel_steps = {
            let mut channel_steps: [f32; 4] = [0.; 4];
            for pixel_channel in 0..pixel_width {
                channel_steps[pixel_channel] =
                    colour_end[pixel_channel] - colour_begin[pixel_channel];
            }
            channel_steps
        };

        // Multiplier for sub pixel step. Value is in the range: `0..pixel_count`.
        let mut pixel_n = 0;
        for y in 0..image_height {
            for x in 0..image_width {
                for pixel_channel in 0..pixel_width {
                    let index = (y * image_width + x) * pixel_width;
                    pixel_data[index] =
                        colour_begin[pixel_channel] + pixel_n as f32 * channel_steps[pixel_channel];
                }
                pixel_n += 1;
            }
        }

        let metadata = TextureMetadata::srgb().with_size(image_width as u16, image_height as u16);

        (metadata, pixel_data)
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{renderer::SpriteRender, Error};
    use amethyst_test::AmethystApplication;

    use super::{ColourSpriteSheetGen, ColourSpriteSheetGenData};

    #[test]
    fn solid_returns_sprite_render_with_colour() -> Result<(), Error> {
        const RED: [f32; 4] = [1., 0.2, 0.1, 1.];

        AmethystApplication::render_base("solid_returns_sprite_render_with_colour", false)
            .with_setup(|world| {
                let sprite_render = {
                    let colour_sprite_gen_data = world.system_data::<ColourSpriteSheetGenData>();
                    ColourSpriteSheetGen::solid(&colour_sprite_gen_data, RED)
                };
                world.add_resource(sprite_render);
            })
            .with_assertion(|world| {
                let sprite_render = &*world.read_resource::<SpriteRender>();

                let ColourSpriteSheetGenData {
                    texture_assets,
                    sprite_sheet_assets,
                    ..
                } = world.system_data::<ColourSpriteSheetGenData>();

                assert_eq!(0, sprite_render.sprite_number);

                let sprite_sheet = sprite_sheet_assets.get(&sprite_render.sprite_sheet);
                assert!(sprite_sheet.is_some());

                let sprite_sheet = sprite_sheet.expect("Expected `SpriteSheet` to exist.");
                assert!(texture_assets.get(&sprite_sheet.texture).is_some());
            })
            .run()
    }
}
