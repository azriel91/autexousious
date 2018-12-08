#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Processes sprite configuration into the loaded sprite model.
//!
//! # Examples
//!
//! ```rust
//! extern crate amethyst;
//! extern crate assets_test;
//! extern crate sprite_loading;
//!
//! use std::path::{Path, PathBuf};
//!
//! use amethyst::ecs::prelude::*;
//! use assets_test::ASSETS_CHAR_BAT_PATH;
//! use sprite_loading::SpriteLoader;
//!
//! fn my_function(world: &mut World) {
//!     let result = SpriteLoader::load(world, &ASSETS_CHAR_BAT_PATH);
//!
//!     assert!(result.is_ok());
//! }
//! ```

use application;

#[macro_use]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::{
    animation::{AnimationFrame, AnimationSequence, SpriteRenderAnimationLoader},
    sprite_animation_handle::SpriteAnimationHandle,
    sprite_loader::SpriteLoader,
};
pub(crate) use crate::{
    sprite_sheet_loader::SpriteSheetLoader, sprite_sheet_mapper::SpriteSheetMapper,
    texture_loader::TextureLoader,
};

mod animation;
mod sprite_animation_handle;
mod sprite_loader;
mod sprite_sheet_loader;
mod sprite_sheet_mapper;
mod texture_loader;

#[cfg(test)]
mod test {
    use amethyst_test::AmethystApplication;
    use assets_test::ASSETS_CHAR_BAT_PATH;

    use super::SpriteLoader;

    #[test]
    fn loads_sprite_sheets_textures_and_mesh() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_sprite_sheets_textures_and_mesh", false)
                .with_assertion(|world| {
                    let result = SpriteLoader::load(world, &ASSETS_CHAR_BAT_PATH);

                    if let Err(e) = result {
                        panic!("Failed to load sprites: {:?}", e); // kcov-ignore
                    } // kcov-ignore
                })
                .run()
                .is_ok()
        );
    }
}
