use amethyst::{
    assets::Handle,
    renderer::{
        sprite::{Sprite, SpriteSheet, TextureCoordinates},
        Texture,
    },
};

use crate::{ColourSpriteSheetParams, SpriteGenParams};

/// Generates `SpriteSheet`s with various methods of texture coordinate
/// calculation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpriteSheetGen {
    /// Generates texture coordinates sitting exactly on the edge.
    Edge,
    /// Generates texture coordinates 0.5 pixels in from the edge.
    ///
    /// This is useful when you don't want *any* part of the pixels adjacent to
    /// the edge pixel from leaking into the render.
    HalfPixel,
}

impl SpriteSheetGen {
    /// Returns a `SpriteSheet` whose sprites' texture coordinates use a grid
    /// layout.
    pub fn generate(
        self,
        texture_handle: Handle<Texture>,
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

    /// Visible for testing.
    pub fn generate_sprites(
        self,
        params: ColourSpriteSheetParams,
        sprite_count: usize,
        image_w: u32,
        image_h: u32,
    ) -> Vec<Sprite> {
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
                    offsets: [0.; 2],
                };
                let sprite = self.sprite_from_pixel_values(sprite_gen_params);

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
    /// This function expects pixel coordinates -- starting from the top left of
    /// the image. X increases to the right, Y increases downwards. Texture
    /// coordinates are calculated from the pixel values.
    ///
    /// # Parameters
    ///
    /// * `sprite_gen_params`: Parameters to generate a sprite.
    pub fn sprite_from_pixel_values(
        self,
        SpriteGenParams {
            image_w,
            image_h,
            sprite_w,
            sprite_h,
            pixel_left,
            pixel_top,
            offsets,
        }: SpriteGenParams,
    ) -> Sprite {
        // Fraction of a pixel to shift inward from the edge of the sprite.
        //
        // `0.` means texture coordinates lie exactly on the pixel edge, which would
        // make a sprite pixel perfect, assuming its position aligns exactly
        // with a screen pixel.
        let edge_shift = match self {
            SpriteSheetGen::Edge => 0.,
            SpriteSheetGen::HalfPixel => 0.5,
        };

        let image_w = image_w as f32;
        let image_h = image_h as f32;
        let offsets = [offsets[0] as f32, offsets[1] as f32];

        let pixel_right = (pixel_left + sprite_w) as f32;
        let pixel_bottom = (pixel_top + sprite_h) as f32;
        let pixel_left = pixel_left as f32;
        let pixel_top = pixel_top as f32;

        // Texture coordinates are expressed as fractions of the position on the image.
        // Y axis texture coordinates start at the bottom of the image, so we have to
        // invert them.
        //
        // For pixel perfect result, the sprite border must be rendered exactly at
        // screen pixel border or use nearest-neighbor sampling.
        // <http://www.mindcontrol.org/~hplus/graphics/opengl-pixel-perfect.html>
        // NOTE: Maybe we should provide an option to round coordinates from `Transform`
        // to nearest integer in `DrawFlat2D` pass before rendering.
        let left = (pixel_left + edge_shift) / image_w;
        let right = (pixel_right - edge_shift) / image_w;
        let top = (pixel_top + edge_shift) / image_h;
        let bottom = (pixel_bottom - edge_shift) / image_h;

        let tex_coords = TextureCoordinates {
            left,
            right,
            bottom,
            top,
        };

        Sprite {
            width: sprite_w as f32,
            height: sprite_h as f32,
            offsets,
            tex_coords,
        }
    }
}
