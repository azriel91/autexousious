#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Processes sprite configuration into the loaded sprite model.
//!
//! # Examples
//!
//! ```rust
//! extern crate amethyst;
//! extern crate game_model;
//! extern crate sprite_loading;
//!
//! use std::path::{Path, PathBuf};
//!
//! use amethyst::ecs::prelude::*;
//! use game_model::config::ConfigRecord;
//! use sprite_loading::SpriteLoader;
//!
//! fn my_function(world: &mut World) {
//!     let texture_index_offset = 0;
//!     let assets_dir = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
//!     let assets_dir = Path::new(&assets_dir);
//!     let bat_path = assets_dir.join("test/object/character/bat");
//!     let config_record = ConfigRecord::new(bat_path);
//!     let result = SpriteLoader::load(world, texture_index_offset, &config_record);
//!
//!     assert!(result.is_ok());
//! }
//! ```

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
#[cfg(not(test))]
extern crate application;
#[cfg(test)]
#[macro_use]
extern crate application;
extern crate game_model;
#[macro_use]
extern crate log;
extern crate sprite_model;

pub(crate) use material_creator::MaterialCreator;
pub use sprite_loader::SpriteLoader;
pub(crate) use sprite_mesh_creator::SpriteMeshCreator;
pub(crate) use sprite_sheet_mapper::SpriteSheetMapper;
pub(crate) use texture_loader::TextureLoader;

mod material_creator;
mod sprite_loader;
mod sprite_mesh_creator;
mod sprite_sheet_mapper;
mod texture_loader;

#[cfg(test)]
mod test {
    use std::path::Path;

    use amethyst_test_support::AmethystApplication;
    use application::resource::dir::assets_dir;
    use game_model::config::ConfigRecord;

    use super::SpriteLoader;

    #[test]
    fn loads_sprite_sheets_textures_and_mesh() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_sprite_sheets_textures_and_mesh", false)
                .with_assertion(|world| {
                    let texture_index_offset = 0;
                    let mut bat_path = assets_dir(Some(development_base_dirs!()))
                        .expect("Expected assets directory to exist.");
                    bat_path.extend(Path::new("test/object/character/bat").iter());
                    let config_record = ConfigRecord::new(bat_path);
                    let result = SpriteLoader::load(world, texture_index_offset, &config_record);

                    if let Err(e) = result {
                        panic!("Failed to load sprites: {:?}", e); // kcov-ignore
                    } // kcov-ignore
                })
                .run()
                .is_ok()
        );
    }
}
