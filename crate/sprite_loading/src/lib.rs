#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes sprite configuration into the loaded sprite model.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::{sprite_loader::SpriteLoader, sprite_loading_bundle::SpriteLoadingBundle};
pub(crate) use crate::{
    sprite_sheet_loader::SpriteSheetLoader, sprite_sheet_mapper::SpriteSheetMapper,
    texture_loader::TextureLoader,
};

mod sprite_loader;
mod sprite_loading_bundle;
mod sprite_sheet_loader;
mod sprite_sheet_mapper;
mod texture_loader;

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::{AssetStorage, Loader, ProgressCounter},
        core::TransformBundle,
        ecs::WorldExt,
        renderer::{types::DefaultBackend, RenderEmptyBundle, SpriteSheet, Texture},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use application::{load_in, resource::Format};
    use assets_test::CHAR_BAT_PATH;
    use sprite_model::config::SpritesDefinition;

    use super::SpriteLoader;

    #[test]
    fn loads_textures_and_sprite_sheets() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_assertion(|world| {
                let sprites_definition = load_in::<SpritesDefinition, _>(
                    &*CHAR_BAT_PATH,
                    "sprites.yaml",
                    Format::Yaml,
                    None,
                )
                .expect("Failed to load sprites_definition.");

                let loader = world.read_resource::<Loader>();
                let texture_assets = world.read_resource::<AssetStorage<Texture>>();
                let sprite_sheet_assets = world.read_resource::<AssetStorage<SpriteSheet>>();

                let result = SpriteLoader::load(
                    &mut ProgressCounter::default(),
                    &loader,
                    &texture_assets,
                    &sprite_sheet_assets,
                    &sprites_definition,
                    &CHAR_BAT_PATH,
                );

                if let Err(e) = result {
                    panic!("Failed to load sprites: {:?}", e); // kcov-ignore
                } // kcov-ignore
            })
            .run_isolated()
    }
}
