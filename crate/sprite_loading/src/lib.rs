#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Processes sprite configuration into the loaded sprite model.
//!
//! # Examples
//!
//! ```rust
//! use std::path::{Path, PathBuf};
//!
//! use amethyst::{
//!     assets::{AssetStorage, Loader},
//!     ecs::World,
//!     renderer::{SpriteSheet, Texture},
//! };
//! use assets_test::ASSETS_CHAR_BAT_PATH;
//! use sprite_loading::SpriteLoader;
//!
//! fn my_function(world: &mut World) {
//!     let loader = world.read_resource::<Loader>();
//!     let texture_assets = world.read_resource::<AssetStorage<Texture>>();
//!     let sprite_sheet_assets = world.read_resource::<AssetStorage<SpriteSheet>>();
//!
//!     let result = SpriteLoader::load(
//!         &loader,
//!         &texture_assets,
//!         &sprite_sheet_assets,
//!         &ASSETS_CHAR_BAT_PATH
//!     );
//!
//!     assert!(result.is_ok());
//! }
//! ```

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
    use amethyst::{
        assets::{AssetStorage, Loader},
        renderer::{SpriteSheet, Texture},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use assets_test::ASSETS_CHAR_BAT_PATH;

    use super::SpriteLoader;

    #[test]
    fn loads_textures_and_sprite_sheets() -> Result<(), Error> {
        AmethystApplication::render_base("loads_textures_and_sprite_sheets", false)
            .with_assertion(|world| {
                let loader = world.read_resource::<Loader>();
                let texture_assets = world.read_resource::<AssetStorage<Texture>>();
                let sprite_sheet_assets = world.read_resource::<AssetStorage<SpriteSheet>>();

                let result = SpriteLoader::load(
                    &loader,
                    &texture_assets,
                    &sprite_sheet_assets,
                    &ASSETS_CHAR_BAT_PATH,
                );

                if let Err(e) = result {
                    panic!("Failed to load sprites: {:?}", e); // kcov-ignore
                } // kcov-ignore
            })
            .run()
    }
}
