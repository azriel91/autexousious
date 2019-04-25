use amethyst::renderer::{Sprite, SpriteSheet, TextureCoordinates, TextureHandle};

use crate::{ColourSpriteSheetParams, SpriteGenParams};

/// Generates `SpriteSheet`s with various methods of texture coordinate calculation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpriteSheetGen {
    /// Generates texture coordinates sitting exactly on the edge.
    Edge,
    /// Generates texture coordinates 0.5 pixels in from the edge.
    ///
    /// This is useful when you don't want *any* part of the pixels adjacent to the edge pixel from
    /// leaking into the render.
    HalfPixel,
}

impl SpriteSheetGen {
    /// Returns a `SpriteSheet` whose sprites' texture coordinates use a grid layout.
    pub fn generate(
        self,
        texture_handle: TextureHandle,
        params: ColourSpriteSheetParams,
        sprite_count: usize,
        image_w: u32,
        image_h: u32,
    ) -> SpriteSheet {
        let sprites = self.generate_sprites(params, sprite_count, image_w, image_h);

        SpriteSheet {
            texture: texture_handle,
            sprites,
        }
    }

    fn generate_sprites(
        self,
        params: ColourSpriteSheetParams,
        sprite_count: usize,
        image_w: u32,
        image_h: u32,
    ) -> Vec<Sprite> {
        let edge_shift = match self {
            SpriteSheetGen::Edge => 0.,
            SpriteSheetGen::HalfPixel => 0.5,
        };

        let mut sprites = Vec::with_capacity(sprite_count);
        let padding_pixels = if params.padded { 1 } else { 0 };
        let offset_w = params.sprite_w + padding_pixels;
        let offset_h = params.sprite_h + padding_pixels;
        'outer: for row in 0..params.row_count {
            for col in 0..params.column_count {
                let offset_x = offset_w * col as u32;
                let offset_y = offset_h * row as u32;
                let sprite_gen_params = SpriteGenParams {
                    image_w: image_w as u32,
                    image_h: image_h as u32,
                    sprite_w: params.sprite_w,
                    sprite_h: params.sprite_h,
                    pixel_left: offset_x,
                    pixel_top: offset_y,
                    offsets: [0; 2],
                    edge_shift,
                };
                let sprite = Self::from_pixel_values(sprite_gen_params);

                sprites.push(sprite);

                if sprites.len() == sprite_count {
                    break 'outer;
                }
            }
        }

        sprites
    }

    /// Creates a `Sprite` from pixel values.
    ///
    /// This function expects pixel coordinates -- starting from the top left of the image. X
    /// increases to the right, Y increases downwards. Texture coordinates are calculated from the
    /// pixel values.
    ///
    /// # Parameters
    ///
    /// * `sprite_gen_params`: Parameters to generate a sprite.
    pub fn from_pixel_values(
        SpriteGenParams {
            image_w,
            image_h,
            sprite_w,
            sprite_h,
            pixel_left,
            pixel_top,
            offsets,
            edge_shift,
        }: SpriteGenParams,
    ) -> Sprite {
        let image_w = image_w as f32;
        let image_h = image_h as f32;
        let offsets = [offsets[0] as f32, offsets[1] as f32];

        let pixel_right = (pixel_left + sprite_w) as f32;
        let pixel_bottom = (pixel_top + sprite_h) as f32;
        let pixel_left = pixel_left as f32;
        let pixel_top = pixel_top as f32;

        // Texture coordinates are expressed as fractions of the position on the image.
        // Y axis texture coordinates start at the bottom of the image, so we have to invert them.
        //
        // For pixel perfect result, the sprite border must be rendered exactly at
        // screen pixel border or use nearest-neighbor sampling.
        // <http://www.mindcontrol.org/~hplus/graphics/opengl-pixel-perfect.html>
        // NOTE: Maybe we should provide an option to round coordinates from `Transform`
        // to nearest integer in `DrawFlat2D` pass before rendering.
        let left = (pixel_left + edge_shift) / image_w;
        let right = (pixel_right - edge_shift) / image_w;
        let top = (image_h - (pixel_top + edge_shift)) / image_h;
        let bottom = (image_h - (pixel_bottom - edge_shift)) / image_h;

        let tex_coords = TextureCoordinates {
            left,
            right,
            top,
            bottom,
        };

        Sprite {
            width: sprite_w as f32,
            height: sprite_h as f32,
            offsets,
            tex_coords,
        }
    }
}

#[cfg(test)]
mod tests {
    use amethyst::renderer::Sprite;

    use super::SpriteSheetGen;
    use crate::ColourSpriteSheetParams;

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
                [0. / 30., 9. / 30., 21. / 40., 40. / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [10. / 30., 19. / 30., 21. / 40., 40. / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [20. / 30., 29. / 30., 21. / 40., 40. / 40.],
            )
                .into(),
            // Sprites bottom row
            (
                (9., 19.),
                [0.; 2],
                [0. / 30., 9. / 30., 1. / 40., 20. / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [10. / 30., 19. / 30., 1. / 40., 20. / 40.],
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
                [0. / 30., 10. / 30., 20. / 40., 40. / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [10. / 30., 20. / 30., 20. / 40., 40. / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [20. / 30., 30. / 30., 20. / 40., 40. / 40.],
            )
                .into(),
            // Sprites bottom row
            (
                (10., 20.),
                [0.; 2],
                [0. / 30., 10. / 30., 0. / 40., 20. / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [10. / 30., 20. / 30., 0. / 40., 20. / 40.],
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
                [0.5 / 30., 8.5 / 30., 21.5 / 40., 39.5 / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [10.5 / 30., 18.5 / 30., 21.5 / 40., 39.5 / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [20.5 / 30., 28.5 / 30., 21.5 / 40., 39.5 / 40.],
            )
                .into(),
            // Sprites bottom row
            (
                (9., 19.),
                [0.; 2],
                [0.5 / 30., 8.5 / 30., 1.5 / 40., 19.5 / 40.],
            )
                .into(),
            (
                (9., 19.),
                [0.; 2],
                [10.5 / 30., 18.5 / 30., 1.5 / 40., 19.5 / 40.],
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
                [0.5 / 30., 9.5 / 30., 20.5 / 40., 39.5 / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [10.5 / 30., 19.5 / 30., 20.5 / 40., 39.5 / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [20.5 / 30., 29.5 / 30., 20.5 / 40., 39.5 / 40.],
            )
                .into(),
            // Sprites bottom row
            (
                (10., 20.),
                [0.; 2],
                [0.5 / 30., 9.5 / 30., 0.5 / 40., 19.5 / 40.],
            )
                .into(),
            (
                (10., 20.),
                [0.; 2],
                [10.5 / 30., 19.5 / 30., 0.5 / 40., 19.5 / 40.],
            )
                .into(),
        ];

        assert_eq!(expected, sprites);
    }
}
