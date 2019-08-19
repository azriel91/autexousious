use std::ptr;

use amethyst::renderer::{
    loaders::load_from_srgba,
    palette::Srgba,
    rendy::{
        hal::image::{Filter, Kind, SamplerInfo, ViewKind, WrapMode},
        texture::{pixel::Rgba8Srgb, TextureBuilder},
    },
    types::TextureData,
    Sprite, SpriteRender, SpriteSheet,
};
use integer_sqrt::IntegerSquareRoot;

use crate::{ColourSpriteSheetGenData, ColourSpriteSheetParams, SpriteSheetGen};

const COLOUR_TRANSPARENT: [f32; 4] = [0f32; 4];

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
            let texture_builder =
                load_from_srgba(Srgba::new(colour[0], colour[1], colour[2], colour[3]));
            let texture_handle =
                loader.load_from_data(TextureData::from(texture_builder), (), &texture_assets);
            let sprite = Sprite::from_pixel_values(1, 1, 1, 1, 0, 0, [0.; 2], false, false);
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

    /// Returns a `SpriteRender` that holds a reference to a gradient spritesheet.
    ///
    /// This generates a sprite for each colour between `colour_begin` and `colour_end` (inclusive).
    /// The number of sprites in the sprite sheet is equal to the `sprite_count` parameter.
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
            // kcov-ignore-start
            panic!(
                "`sprite_count` must be at least 2, received: `{}`.",
                sprite_count
            );
            // kcov-ignore-end
        }

        let sprite_sheet_handle = {
            let column_count = sprite_count.integer_sqrt();
            let row_count = {
                let needs_buffer = column_count * column_count < sprite_count;
                sprite_count / column_count + if needs_buffer { 1 } else { 0 }
            };
            let params = ColourSpriteSheetParams {
                sprite_w: 1,
                sprite_h: 1,
                padded: true,
                row_count,
                column_count,
            };

            let (pixel_data, image_w, image_h) =
                Self::gradient_colours(params, colour_begin, colour_end, sprite_count);

            let pixel_data = pixel_data
                .into_iter()
                .map(|[red, green, blue, alpha]| {
                    Rgba8Srgb::from(Srgba::new(red, green, blue, alpha))
                })
                .collect::<Vec<Rgba8Srgb>>();

            let texture_builder = TextureBuilder::new()
                .with_kind(Kind::D2(image_w, image_h, 1, 1))
                .with_view_kind(ViewKind::D2)
                .with_data_width(image_w)
                .with_data_height(image_h)
                .with_sampler_info(SamplerInfo::new(Filter::Linear, WrapMode::Clamp))
                .with_data(pixel_data);
            let texture_data = texture_builder.into();

            let texture_handle = loader.load_from_data(texture_data, (), &texture_assets);
            let sprite_sheet = SpriteSheetGen::HalfPixel.generate(
                texture_handle,
                params,
                sprite_count,
                image_w,
                image_h,
            );

            loader.load_from_data(sprite_sheet, (), sprite_sheet_assets)
        };

        SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        }
    }

    fn gradient_colours(
        ColourSpriteSheetParams {
            sprite_w,
            sprite_h,
            padded,
            row_count,
            column_count,
        }: ColourSpriteSheetParams,
        colour_begin: [f32; 4],
        colour_end: [f32; 4],
        sprite_count: usize,
    ) -> (Vec<[f32; 4]>, u32, u32) {
        // Pixel count.
        let padding_pixels = if padded { 1 } else { 0 };
        let sprite_w_pad = sprite_w + padding_pixels;
        let sprite_h_pad = sprite_h + padding_pixels;
        let image_width = sprite_w_pad as usize * column_count;
        let image_height = sprite_h_pad as usize * row_count;
        let pixel_count = image_width * image_height;

        // Element count.
        let capacity = pixel_count;
        let mut pixel_data = vec![[0f32; 4]; capacity];

        // Calculate colour values.
        //
        // Pixel coordinates are used, so Y increases downwards.

        let channel_steps = Self::channel_steps(sprite_count, colour_begin, colour_end);

        (0..row_count).for_each(|sprite_row| {
            // 1. Build up a row of pixels
            // 2. Duplicate the row `sprite_h` times
            // 3. Add padding pixels if necessary
            // 4. Repeat

            let capacity = sprite_w_pad as usize * column_count;
            let pixel_row =
                (0..column_count).fold(vec![[0f32; 4]; capacity], |mut pixel_row, sprite_col| {
                    // For each sprite column, generate sprite_w colour pixels, and maybe
                    // 1 padding pixel.

                    let sprite_n = sprite_row * column_count + sprite_col;

                    // Calculate sprite colour
                    let sprite_colour = if sprite_n < sprite_count {
                        let mut colour = COLOUR_TRANSPARENT;

                        macro_rules! calculate_colour {
                            ($channel:tt) => {
                                colour[$channel] = colour_begin[$channel]
                                    + sprite_n as f32 * channel_steps[$channel];
                            };
                        }

                        calculate_colour!(0);
                        calculate_colour!(1);
                        calculate_colour!(2);
                        calculate_colour!(3);

                        colour
                    } else {
                        COLOUR_TRANSPARENT
                    };

                    // Fill in `sprite_w` pixels with `sprite_colour`
                    (0..sprite_w).for_each(|pixel_n| {
                        // `pixel_n` is the pixel number, not the colour channel index in
                        // `pixel_row`.
                        let pixel_index = sprite_col * sprite_w_pad as usize + pixel_n as usize;

                        unsafe {
                            ptr::copy_nonoverlapping(
                                sprite_colour.as_ptr(),
                                pixel_row[pixel_index].as_mut_ptr(),
                                4,
                            )
                        }
                    });

                    // Not necessary to add padding pixels explicitly -- that is handled by the
                    // initialization with `capacity`.

                    pixel_row
                });

            // Copy pixel row `sprite_h` times.
            let pixel_data_row_offset =
                sprite_row * sprite_h_pad as usize * sprite_w_pad as usize * column_count;
            let pixel_row_len = pixel_row.len();
            (0..sprite_h).for_each(|pixel_row_n| unsafe {
                ptr::copy_nonoverlapping(
                    pixel_row.as_ptr(),
                    pixel_data
                        .as_mut_ptr()
                        .add(pixel_data_row_offset + pixel_row_n as usize * pixel_row_len),
                    pixel_row_len,
                )
            });

            // Not necessary to add padding pixels explicitly -- that is handled by the
            // initialization with `capacity`.
        });

        let image_width = image_width as u32;
        let image_height = image_height as u32;

        (pixel_data, image_width, image_height)
    }

    fn channel_steps(
        sprite_count: usize,
        colour_begin: [f32; 4],
        colour_end: [f32; 4],
    ) -> [f32; 4] {
        let mut channel_steps = [0f32; 4];

        // Example:
        //
        // `sprite_count`: 6
        // `begin`: 3
        // `end`: 8
        //
        // Expected: 3, 4, 5, 6, 7, 8
        //
        // Step is 1, which is:
        // = 5 / 5
        // = (8 - 3) / (6 - 1)
        // = (end - start) / (sprite_count - 1)

        macro_rules! calculate_channel_step {
            ($channel:tt) => {
                let channel_diff = colour_end[$channel] - colour_begin[$channel];
                channel_steps[$channel] = channel_diff / (sprite_count - 1) as f32;
            };
        }

        calculate_channel_step!(0);
        calculate_channel_step!(1);
        calculate_channel_step!(2);
        calculate_channel_step!(3);

        channel_steps
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        core::TransformBundle,
        renderer::{sprite::SpriteRender, types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use approx::assert_relative_eq;

    use super::ColourSpriteSheetGen;
    use crate::{ColourSpriteSheetGenData, ColourSpriteSheetParams};

    #[test]
    fn solid_returns_sprite_render() -> Result<(), Error> {
        const RED: [f32; 4] = [1., 0.2, 0.1, 1.];

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_setup(|world| {
                let sprite_render = {
                    let colour_sprite_gen_data = world.system_data::<ColourSpriteSheetGenData>();
                    ColourSpriteSheetGen::solid(&colour_sprite_gen_data, RED)
                };
                world.insert(sprite_render);
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
            .run_isolated()
    }

    #[test]
    fn gradient_returns_sprite_render() -> Result<(), Error> {
        const COLOUR_BEGIN: [f32; 4] = [1., 0., 0., 0.5];
        const COLOUR_END: [f32; 4] = [0., 1., 0., 1.];

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_setup(|world| {
                let sprite_render = {
                    let colour_sprite_gen_data = world.system_data::<ColourSpriteSheetGenData>();
                    ColourSpriteSheetGen::gradient(
                        &colour_sprite_gen_data,
                        COLOUR_BEGIN,
                        COLOUR_END,
                        5,
                    )
                };
                world.insert(sprite_render);
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
            .run_isolated()
    }

    #[test]
    fn gradient_colours_generates_pixel_data_1x1_sprite_padded() {
        let colour_sprite_sheet_params = ColourSpriteSheetParams {
            sprite_w: 1,
            sprite_h: 1,
            padded: true,
            row_count: 2,
            column_count: 3,
        };
        let colour_begin = [1., 0.2, 0., 0.6];
        let colour_end = [0.2, 1., 0., 1.];
        let sprite_count = 5;

        let (colours, _, _) = ColourSpriteSheetGen::gradient_colours(
            colour_sprite_sheet_params,
            colour_begin,
            colour_end,
            sprite_count,
        );

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[0]);
        assert_relative_eq!([0.; 4][..], colours[1]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[2]);
        assert_relative_eq!([0.; 4][..], colours[3]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[4]);
        assert_relative_eq!([0.; 4][..], colours[5]);

        // Padding row.
        // row_length
        //     = (1 sprite_pixel + 1 padding_pixel) * column_count * 4 channels
        //     = 2 * 3 * 4
        //     = 24
        // 1 padding pixel * row_length
        assert_relative_eq!([0.; 4][..], colours[6]);
        assert_relative_eq!([0.; 4][..], colours[7]);
        assert_relative_eq!([0.; 4][..], colours[8]);
        assert_relative_eq!([0.; 4][..], colours[9]);
        assert_relative_eq!([0.; 4][..], colours[10]);
        assert_relative_eq!([0.; 4][..], colours[11]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[12]);
        assert_relative_eq!([0.; 4][..], colours[13]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[14]);
        assert_relative_eq!([0.; 4][..], colours[15]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[16]);
        assert_relative_eq!([0.; 4][..], colours[17]);

        assert_relative_eq!([0.; 4][..], colours[18]);
        assert_relative_eq!([0.; 4][..], colours[19]);
        assert_relative_eq!([0.; 4][..], colours[20]);
        assert_relative_eq!([0.; 4][..], colours[21]);
        assert_relative_eq!([0.; 4][..], colours[22]);
        assert_relative_eq!([0.; 4][..], colours[23]);
    }

    #[test]
    fn gradient_colours_generates_pixel_data_2x1_sprite_padded() {
        let colour_sprite_sheet_params = ColourSpriteSheetParams {
            sprite_w: 2,
            sprite_h: 1,
            padded: true,
            row_count: 2,
            column_count: 3,
        };
        let colour_begin = [1., 0.2, 0., 0.6];
        let colour_end = [0.2, 1., 0., 1.];
        let sprite_count = 5;

        let (colours, _, _) = ColourSpriteSheetGen::gradient_colours(
            colour_sprite_sheet_params,
            colour_begin,
            colour_end,
            sprite_count,
        );

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[0]);
        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[1]);
        assert_relative_eq!([0.; 4][..], colours[2]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[3]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[4]);
        assert_relative_eq!([0.; 4][..], colours[5]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[6]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[7]);
        assert_relative_eq!([0.; 4][..], colours[8]);

        // Padding row.
        // row_length
        //     = (2 sprite_pixels + 1 padding_pixel) * column_count * 4 channels
        //     = 3 * 3 * 4
        //     = 36
        // 1 padding pixel * row_length
        assert_relative_eq!([0.; 4][..], colours[9]);
        assert_relative_eq!([0.; 4][..], colours[10]);
        assert_relative_eq!([0.; 4][..], colours[11]);
        assert_relative_eq!([0.; 4][..], colours[12]);
        assert_relative_eq!([0.; 4][..], colours[13]);
        assert_relative_eq!([0.; 4][..], colours[14]);
        assert_relative_eq!([0.; 4][..], colours[15]);
        assert_relative_eq!([0.; 4][..], colours[16]);
        assert_relative_eq!([0.; 4][..], colours[17]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[18]);
        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[19]);
        assert_relative_eq!([0.; 4][..], colours[20]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[21]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[22]);
        assert_relative_eq!([0.; 4][..], colours[23]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[24]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[25]);
        assert_relative_eq!([0.; 4][..], colours[26]);

        assert_relative_eq!([0.; 4][..], colours[27]);
        assert_relative_eq!([0.; 4][..], colours[28]);
        assert_relative_eq!([0.; 4][..], colours[29]);
        assert_relative_eq!([0.; 4][..], colours[30]);
        assert_relative_eq!([0.; 4][..], colours[31]);
        assert_relative_eq!([0.; 4][..], colours[32]);
        assert_relative_eq!([0.; 4][..], colours[33]);
        assert_relative_eq!([0.; 4][..], colours[34]);
        assert_relative_eq!([0.; 4][..], colours[35]);
    }

    #[test]
    fn gradient_colours_generates_pixel_data_1x2_sprite_padded() {
        let colour_sprite_sheet_params = ColourSpriteSheetParams {
            sprite_w: 1,
            sprite_h: 2,
            padded: true,
            row_count: 2,
            column_count: 3,
        };
        let colour_begin = [1., 0.2, 0., 0.6];
        let colour_end = [0.2, 1., 0., 1.];
        let sprite_count = 5;

        let (colours, _, _) = ColourSpriteSheetGen::gradient_colours(
            colour_sprite_sheet_params,
            colour_begin,
            colour_end,
            sprite_count,
        );

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[0]);
        assert_relative_eq!([0.; 4][..], colours[1]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[2]);
        assert_relative_eq!([0.; 4][..], colours[3]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[4]);
        assert_relative_eq!([0.; 4][..], colours[5]);

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[6]);
        assert_relative_eq!([0.; 4][..], colours[7]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[8]);
        assert_relative_eq!([0.; 4][..], colours[9]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[10]);
        assert_relative_eq!([0.; 4][..], colours[11]);

        // Padding row.
        // row_length
        //     = (1 sprite_pixel + 1 padding_pixel) * column_count
        //     = 2 * 3
        //     = 6
        // 1 padding pixel * row_length
        assert_relative_eq!([0.; 4][..], colours[12]);
        assert_relative_eq!([0.; 4][..], colours[13]);
        assert_relative_eq!([0.; 4][..], colours[14]);
        assert_relative_eq!([0.; 4][..], colours[15]);
        assert_relative_eq!([0.; 4][..], colours[16]);
        assert_relative_eq!([0.; 4][..], colours[17]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[18]);
        assert_relative_eq!([0.; 4][..], colours[19]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[20]);
        assert_relative_eq!([0.; 4][..], colours[21]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[22]);
        assert_relative_eq!([0.; 4][..], colours[23]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[24]);
        assert_relative_eq!([0.; 4][..], colours[25]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[26]);
        assert_relative_eq!([0.; 4][..], colours[27]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[28]);
        assert_relative_eq!([0.; 4][..], colours[29]);

        assert_relative_eq!([0.; 4][..], colours[30]);
        assert_relative_eq!([0.; 4][..], colours[31]);
        assert_relative_eq!([0.; 4][..], colours[32]);
        assert_relative_eq!([0.; 4][..], colours[33]);
        assert_relative_eq!([0.; 4][..], colours[34]);
        assert_relative_eq!([0.; 4][..], colours[35]);
    }

    #[test]
    fn gradient_colours_generates_pixel_data_2x2_sprite_padded() {
        let colour_sprite_sheet_params = ColourSpriteSheetParams {
            sprite_w: 2,
            sprite_h: 2,
            padded: true,
            row_count: 2,
            column_count: 3,
        };
        let colour_begin = [1., 0.2, 0., 0.6];
        let colour_end = [0.2, 1., 0., 1.];
        let sprite_count = 5;

        let (colours, _, _) = ColourSpriteSheetGen::gradient_colours(
            colour_sprite_sheet_params,
            colour_begin,
            colour_end,
            sprite_count,
        );

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[0]);
        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[1]);
        assert_relative_eq!([0.; 4][..], colours[2]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[3]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[4]);
        assert_relative_eq!([0.; 4][..], colours[5]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[6]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[7]);
        assert_relative_eq!([0.; 4][..], colours[8]);

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[9]);
        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[10]);
        assert_relative_eq!([0.; 4][..], colours[11]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[12]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[13]);
        assert_relative_eq!([0.; 4][..], colours[14]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[15]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[16]);
        assert_relative_eq!([0.; 4][..], colours[17]);

        // Padding row.
        // row_length
        //     = (2 sprite_pixels + 1 padding_pixel) * column_count * 4 channels
        //     = 3 * 3 * 4
        //     = 36
        // 1 padding pixel * row_length
        assert_relative_eq!([0.; 4][..], colours[18]);
        assert_relative_eq!([0.; 4][..], colours[19]);
        assert_relative_eq!([0.; 4][..], colours[20]);
        assert_relative_eq!([0.; 4][..], colours[21]);
        assert_relative_eq!([0.; 4][..], colours[22]);
        assert_relative_eq!([0.; 4][..], colours[23]);
        assert_relative_eq!([0.; 4][..], colours[24]);
        assert_relative_eq!([0.; 4][..], colours[25]);
        assert_relative_eq!([0.; 4][..], colours[26]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[27]);
        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[28]);
        assert_relative_eq!([0.; 4][..], colours[29]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[30]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[31]);
        assert_relative_eq!([0.; 4][..], colours[32]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[33]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[34]);
        assert_relative_eq!([0.; 4][..], colours[35]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[36]);
        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[37]);
        assert_relative_eq!([0.; 4][..], colours[38]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[39]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[40]);
        assert_relative_eq!([0.; 4][..], colours[41]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[42]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[43]);
        assert_relative_eq!([0.; 4][..], colours[44]);

        assert_relative_eq!([0.; 4][..], colours[45]);
        assert_relative_eq!([0.; 4][..], colours[46]);
        assert_relative_eq!([0.; 4][..], colours[47]);
        assert_relative_eq!([0.; 4][..], colours[48]);
        assert_relative_eq!([0.; 4][..], colours[49]);
        assert_relative_eq!([0.; 4][..], colours[50]);
        assert_relative_eq!([0.; 4][..], colours[51]);
        assert_relative_eq!([0.; 4][..], colours[52]);
        assert_relative_eq!([0.; 4][..], colours[53]);
    }

    #[test]
    fn gradient_colours_generates_pixel_data_1x1_sprite_unpadded() {
        let colour_sprite_sheet_params = ColourSpriteSheetParams {
            sprite_w: 1,
            sprite_h: 1,
            padded: false,
            row_count: 2,
            column_count: 3,
        };
        let colour_begin = [1., 0.2, 0., 0.6];
        let colour_end = [0.2, 1., 0., 1.];
        let sprite_count = 5;

        let (colours, _, _) = ColourSpriteSheetGen::gradient_colours(
            colour_sprite_sheet_params,
            colour_begin,
            colour_end,
            sprite_count,
        );

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[0]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[1]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[2]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[3]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[4]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[5]);
    }

    #[test]
    fn gradient_colours_generates_pixel_data_2x1_sprite_unpadded() {
        let colour_sprite_sheet_params = ColourSpriteSheetParams {
            sprite_w: 2,
            sprite_h: 1,
            padded: false,
            row_count: 2,
            column_count: 3,
        };
        let colour_begin = [1., 0.2, 0., 0.6];
        let colour_end = [0.2, 1., 0., 1.];
        let sprite_count = 5;

        let (colours, _, _) = ColourSpriteSheetGen::gradient_colours(
            colour_sprite_sheet_params,
            colour_begin,
            colour_end,
            sprite_count,
        );

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[0]);
        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[1]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[2]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[3]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[4]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[5]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[6]);
        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[7]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[8]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[9]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[10]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[11]);
    }

    #[test]
    fn gradient_colours_generates_pixel_data_1x2_sprite_unpadded() {
        let colour_sprite_sheet_params = ColourSpriteSheetParams {
            sprite_w: 1,
            sprite_h: 2,
            padded: false,
            row_count: 2,
            column_count: 3,
        };
        let colour_begin = [1., 0.2, 0., 0.6];
        let colour_end = [0.2, 1., 0., 1.];
        let sprite_count = 5;

        let (colours, _, _) = ColourSpriteSheetGen::gradient_colours(
            colour_sprite_sheet_params,
            colour_begin,
            colour_end,
            sprite_count,
        );

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[0]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[1]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[2]);

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[3]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[4]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[5]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[6]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[7]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[8]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[9]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[10]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[11]);
    }

    #[test]
    fn gradient_colours_generates_pixel_data_2x2_sprite_unpadded() {
        let colour_sprite_sheet_params = ColourSpriteSheetParams {
            sprite_w: 2,
            sprite_h: 2,
            padded: false,
            row_count: 2,
            column_count: 3,
        };
        let colour_begin = [1., 0.2, 0., 0.6];
        let colour_end = [0.2, 1., 0., 1.];
        let sprite_count = 5;

        let (colours, _, _) = ColourSpriteSheetGen::gradient_colours(
            colour_sprite_sheet_params,
            colour_begin,
            colour_end,
            sprite_count,
        );

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[0]);
        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[1]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[2]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[3]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[4]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[5]);

        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[6]);
        assert_relative_eq!([1.0, 0.2, 0.0, 0.6][..], colours[7]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[8]);
        assert_relative_eq!([0.8, 0.4, 0.0, 0.7][..], colours[9]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[10]);
        assert_relative_eq!([0.6, 0.6, 0.0, 0.8][..], colours[11]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[12]);
        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[13]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[14]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[15]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[16]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[17]);

        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[18]);
        assert_relative_eq!([0.4, 0.8, 0.0, 0.9][..], colours[19]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[20]);
        assert_relative_eq!([0.2, 1.0, 0.0, 1.0][..], colours[21]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[22]);
        assert_relative_eq!([0.0, 0.0, 0.0, 0.0][..], colours[23]);
    }

    #[test]
    fn channel_steps_calculates_step_correctly() {
        let sprite_count = 6;
        let colour_begin = [1., 0., 0., 0.5];
        let colour_end = [0., 1., 0., 1.];
        assert_eq!(
            [-0.2, 0.2, 0., 0.1],
            ColourSpriteSheetGen::channel_steps(sprite_count, colour_begin, colour_end,)
        )
    }
}
