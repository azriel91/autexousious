#[cfg(test)]
mod tests {
    use amethyst::{
        core::TransformBundle,
        ecs::WorldExt,
        renderer::{sprite::SpriteRender, types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use approx::assert_relative_eq;

    use asset_gfx_gen::{ColourSpriteSheetGen, ColourSpriteSheetGenData, ColourSpriteSheetParams};

    #[test]
    fn solid_returns_sprite_render() -> Result<(), Error> {
        const RED: [f32; 4] = [1., 0.2, 0.1, 1.];

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle_event_fn(|event_loop| RenderEmptyBundle::<DefaultBackend>::new(event_loop))
            .with_effect(|world| {
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
            .with_bundle_event_fn(|event_loop| RenderEmptyBundle::<DefaultBackend>::new(event_loop))
            .with_effect(|world| {
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
