#[cfg(test)]
mod tests {
    use amethyst::renderer::Sprite;
    use pretty_assertions::assert_eq;

    use asset_gfx_gen::{ColourSpriteSheetParams, SpriteSheetGen};

    #[test]
    fn generates_edge_texture_coordinates_padded() {
        let params = ColourSpriteSheetParams {
            sprite_w: 9,
            sprite_h: 19,
            padded: true,
            row_count: 2,
            column_count: 3,
        };
        let sprite_count = 5;
        let image_w = 30;
        let image_h = 40;
        let sprites = SpriteSheetGen::Edge.generate_sprites(params, sprite_count, image_w, image_h);

        let expected: Vec<Sprite> = vec![
            // Sprites top row
            (
                (9., 19.),
                [0.; 2],
                [0. / 30., 9. / 30., 19. / 40., 0. / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [10. / 30., 19. / 30., 19. / 40., 0. / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [20. / 30., 29. / 30., 19. / 40., 0. / 40.],
            )
                .into(),
            // Sprites bottom row
            (
                (9., 19.),
                [0.; 2],
                [0. / 30., 9. / 30., 39. / 40., 20. / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [10. / 30., 19. / 30., 39. / 40., 20. / 40.],
            )
                .into(),
        ];

        assert_eq!(expected, sprites);
    }

    #[test]
    fn generates_edge_texture_coordinates_unpadded() {
        let params = ColourSpriteSheetParams {
            sprite_w: 10,
            sprite_h: 20,
            padded: false,
            row_count: 2,
            column_count: 3,
        };
        let sprite_count = 5;
        let image_w = 30;
        let image_h = 40;
        let sprites = SpriteSheetGen::Edge.generate_sprites(params, sprite_count, image_w, image_h);

        let expected: Vec<Sprite> = vec![
            // Sprites top row
            (
                (10., 20.),
                [0.; 2],
                [0. / 30., 10. / 30., 20. / 40., 0. / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [10. / 30., 20. / 30., 20. / 40., 0. / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [20. / 30., 30. / 30., 20. / 40., 0. / 40.],
            )
                .into(),
            // Sprites bottom row
            (
                (10., 20.),
                [0.; 2],
                [0. / 30., 10. / 30., 40. / 40., 20. / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [10. / 30., 20. / 30., 40. / 40., 20. / 40.],
            )
                .into(),
        ];

        assert_eq!(expected, sprites);
    }

    #[test]
    fn generates_half_pixel_texture_coordinates_padded() {
        let params = ColourSpriteSheetParams {
            sprite_w: 9,
            sprite_h: 19,
            padded: true,
            row_count: 2,
            column_count: 3,
        };
        let sprite_count = 5;
        let image_w = 30;
        let image_h = 40;
        let sprites =
            SpriteSheetGen::HalfPixel.generate_sprites(params, sprite_count, image_w, image_h);

        let expected: Vec<Sprite> = vec![
            // Sprites top row
            (
                (9., 19.),
                [0.; 2],
                [0.5 / 30., 8.5 / 30., 18.5 / 40., 0.5 / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [10.5 / 30., 18.5 / 30., 18.5 / 40., 0.5 / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [20.5 / 30., 28.5 / 30., 18.5 / 40., 0.5 / 40.],
            )
                .into(),
            // Sprites bottom row
            (
                (9., 19.),
                [0.; 2],
                [0.5 / 30., 8.5 / 30., 38.5 / 40., 20.5 / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [10.5 / 30., 18.5 / 30., 38.5 / 40., 20.5 / 40.],
            )
                .into(),
        ];

        assert_eq!(expected, sprites);
    }

    #[test]
    fn generates_half_pixel_texture_coordinates_unpadded() {
        let params = ColourSpriteSheetParams {
            sprite_w: 10,
            sprite_h: 20,
            padded: false,
            row_count: 2,
            column_count: 3,
        };
        let sprite_count = 5;
        let image_w = 30;
        let image_h = 40;
        let sprites =
            SpriteSheetGen::HalfPixel.generate_sprites(params, sprite_count, image_w, image_h);

        let expected: Vec<Sprite> = vec![
            // Sprites top row
            (
                (10., 20.),
                [0.; 2],
                [0.5 / 30., 9.5 / 30., 19.5 / 40., 0.5 / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [10.5 / 30., 19.5 / 30., 19.5 / 40., 0.5 / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [20.5 / 30., 29.5 / 30., 19.5 / 40., 0.5 / 40.],
            )
                .into(),
            // Sprites bottom row
            (
                (10., 20.),
                [0.; 2],
                [0.5 / 30., 9.5 / 30., 39.5 / 40., 20.5 / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [10.5 / 30., 19.5 / 30., 39.5 / 40., 20.5 / 40.],
            )
                .into(),
        ];

        assert_eq!(expected, sprites);
    }
}
