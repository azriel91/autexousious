use std::path::Path;

use amethyst::{
    assets::{Loader, Prefab, PrefabLoader},
    ecs::World,
    renderer::SpriteSheetHandle,
    Error,
};
use application::{load_in, Format};
use character_model::config::CharacterDefinition;

use object_model::config::ObjectAssetData;

use crate::{CharacterPrefab, CharacterPrefabHandle};

/// Loads `Character`s from configuration.
#[derive(Debug)]
pub struct CharacterLoader;

impl CharacterLoader {
    /// Returns the loaded `Character` model defined by character configuration.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load character into.
    /// * `path`: Path to the character asset directory.
    pub fn load(
        world: &mut World,
        path: &Path,
        sprite_sheet_handles: Vec<SpriteSheetHandle>,
    ) -> Result<CharacterPrefabHandle, Error> {
        let definition_handle = {
            let character_definition =
                load_in::<CharacterDefinition, _>(&path, "object.toml", Format::Toml, None)?;

            let loader = world.read_resource::<Loader>();
            loader.load_from_data(character_definition, (), &world.read_resource())
        };

        let object_asset_data = ObjectAssetData::new(definition_handle, sprite_sheet_handles);
        let character_prefab = CharacterPrefab::new(object_asset_data);

        let character_prefab_handle = world.exec(|loader: PrefabLoader<'_, CharacterPrefab>| {
            loader.load_from_data(Prefab::new_main(character_prefab), ())
        });
        Ok(character_prefab_handle)
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        assets::{AssetStorage, Loader, Prefab, ProgressCounter},
        renderer::{SpriteSheet, Texture},
        Error, State, StateData, Trans,
    };
    use amethyst_test::{AmethystApplication, GameUpdate};
    use application::{load_in, resource::Format};
    use asset_model::config::AssetRecord;
    use assets_test::{ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};
    use collision_loading::CollisionLoadingBundle;
    use sequence_loading::SequenceLoadingBundle;
    use sprite_loading::SpriteLoader;
    use sprite_model::config::SpritesDefinition;

    use super::CharacterLoader;
    use crate::{CharacterLoadingBundle, CharacterPrefab, CharacterPrefabHandle};

    #[test]
    fn loads_character() -> Result<(), Error> {
        AmethystApplication::render_base("loads_character", false)
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_effect(|world| {
                let asset_record =
                    AssetRecord::new(ASSETS_CHAR_BAT_SLUG.clone(), ASSETS_CHAR_BAT_PATH.clone());

                let sprites_definition = load_in::<SpritesDefinition, _>(
                    &asset_record.path,
                    "sprites.toml",
                    Format::Toml,
                    None,
                )
                .expect("Failed to load sprites_definition.");

                // TODO: <https://gitlab.com/azriel91/autexousious/issues/94>
                let sprite_sheet_handles = {
                    let loader = &world.read_resource::<Loader>();
                    let texture_assets = &world.read_resource::<AssetStorage<Texture>>();
                    let sprite_sheet_assets = &world.read_resource::<AssetStorage<SpriteSheet>>();

                    SpriteLoader::load(
                        &mut ProgressCounter::default(),
                        loader,
                        texture_assets,
                        sprite_sheet_assets,
                        &sprites_definition,
                        &asset_record.path,
                    )
                    .expect("Failed to load sprites.")
                };

                let character_prefab_handle =
                    CharacterLoader::load(world, &asset_record.path, sprite_sheet_handles)
                        .expect("Failed to load character.");

                world.add_resource(character_prefab_handle);
            })
            .with_state(|| RetryAssertion(20))
            .run()
    }

    #[derive(Debug)]
    struct RetryAssertion(u32);
    impl<T, E> State<T, E> for RetryAssertion
    where
        T: GameUpdate,
        E: Send + Sync + 'static,
    {
        fn update(&mut self, data: StateData<'_, T>) -> Trans<T, E> {
            data.data.update(&data.world);

            let character_prefab_handle = &data.world.read_resource::<CharacterPrefabHandle>();
            let character_prefab_assets = data
                .world
                .read_resource::<AssetStorage<Prefab<CharacterPrefab>>>();

            if character_prefab_assets
                .get(character_prefab_handle)
                .is_some()
            {
                Trans::Pop
            } else if self.0 > 0 {
                self.0 -= 1;
                Trans::Pop
            } else {
                panic!("CharacterPrefab failed to load within given retry limit.")
            }
        }
    }
}
