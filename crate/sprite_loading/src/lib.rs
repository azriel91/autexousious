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
//!     let texture_index_offset = 0;
//!     let result = SpriteLoader::load(world, texture_index_offset, &ASSETS_CHAR_BAT_PATH);
//!
//!     assert!(result.is_ok());
//! }
//! ```

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
extern crate application;
#[cfg(test)]
extern crate assets_test;
#[cfg(test)]
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate sprite_model;

pub use animation::{AnimationFrame, AnimationSequence, SpriteRenderAnimationLoader};
pub use sprite_animation_handle::SpriteAnimationHandle;
pub use sprite_loader::SpriteLoader;
pub(crate) use sprite_sheet_loader::SpriteSheetLoader;
pub(crate) use sprite_sheet_mapper::SpriteSheetMapper;
pub(crate) use texture_loader::TextureLoader;

mod animation;
mod sprite_animation_handle;
mod sprite_loader;
mod sprite_sheet_loader;
mod sprite_sheet_mapper;
mod texture_loader;

#[cfg(test)]
mod test {
    use amethyst_test_support::AmethystApplication;
    use assets_test::ASSETS_CHAR_BAT_PATH;

    use super::SpriteLoader;

    #[test]
    fn loads_sprite_sheets_textures_and_mesh() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_sprite_sheets_textures_and_mesh", false)
                .with_assertion(|world| {
                    let texture_index_offset = 0;
                    let result =
                        SpriteLoader::load(world, texture_index_offset, &ASSETS_CHAR_BAT_PATH);

                    if let Err(e) = result {
                        panic!("Failed to load sprites: {:?}", e); // kcov-ignore
                    } // kcov-ignore
                })
                .run()
                .is_ok()
        );
    }
}
