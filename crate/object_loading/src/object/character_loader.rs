use amethyst::{assets::Loader, ecs::World, renderer::SpriteSheetHandle, Error};
use application::{load_in, Format};
use asset_model::config::AssetRecord;
use character_model::{
    config::CharacterDefinition,
    loaded::{Character, CharacterHandle},
};
use object_model::config::ObjectAssetData;

/// Loads `Character`s from configuration.
#[derive(Debug)]
pub struct CharacterLoader;

impl CharacterLoader {
    /// Returns the loaded `Character` model defined by character configuration.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `asset_record`: Entry of the object's configuration.
    pub fn load(
        world: &World,
        asset_record: &AssetRecord,
        sprite_sheet_handles: Vec<SpriteSheetHandle>,
    ) -> Result<CharacterHandle, Error> {
        let character_definition = load_in::<CharacterDefinition, _>(
            &asset_record.path,
            "object.toml",
            Format::Toml,
            None,
        )?;

        let loader = world.read_resource::<Loader>();
        let definition_handle =
            loader.load_from_data(character_definition, (), &world.read_resource());

        let object_asset_data = ObjectAssetData::new(definition_handle, sprite_sheet_handles);

        let wrapper_handle = loader.load_from_data(object_asset_data, (), &world.read_resource());
        let character = Character::new(wrapper_handle);

        let character_handle = loader.load_from_data(character, (), &world.read_resource());
        Ok(character_handle)
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        animation::AnimationBundle,
        assets::{AssetStorage, Loader, Processor},
        renderer::{SpriteSheet, Texture},
    };
    use amethyst_test::prelude::*;
    use application::{load_in, resource::Format};
    use asset_model::config::AssetRecord;
    use assets_test::{ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};
    use character_model::{
        config::CharacterSequenceId,
        loaded::{Character, CharacterHandle},
    };
    use collision_loading::CollisionLoadingBundle;
    use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
    use sprite_loading::SpriteLoader;
    use sprite_model::config::SpritesDefinition;
    use typename::TypeName;

    use super::CharacterLoader;
    use crate::ObjectDefinitionToWrapperProcessor;

    #[test]
    fn loads_character() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_character", false)
                .with_bundle(
                    AnimationBundle::<CharacterSequenceId, BodyFrameActiveHandle>::new(
                        "character_body_frame_acs",
                        "character_body_frame_sis",
                    )
                )
                .with_bundle(AnimationBundle::<
                    CharacterSequenceId,
                    InteractionFrameActiveHandle,
                >::new(
                    "character_interaction_frame_acs",
                    "character_interaction_frame_sis",
                ))
                .with_bundle(CollisionLoadingBundle::new())
                .with_system(
                    ObjectDefinitionToWrapperProcessor::<Character>::new(),
                    ObjectDefinitionToWrapperProcessor::<Character>::type_name(),
                    &[]
                )
                .with_system(Processor::<Character>::new(), "character_processor", &[])
                .with_effect(|world| {
                    let asset_record = AssetRecord::new(
                        ASSETS_CHAR_BAT_SLUG.clone(),
                        ASSETS_CHAR_BAT_PATH.clone(),
                    );

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
                        let sprite_sheet_assets =
                            &world.read_resource::<AssetStorage<SpriteSheet>>();

                        SpriteLoader::load(
                            loader,
                            texture_assets,
                            sprite_sheet_assets,
                            &sprites_definition,
                            &asset_record.path,
                        )
                        .expect("Failed to load sprites.")
                    };

                    let character_handle =
                        CharacterLoader::load(world, &asset_record, sprite_sheet_handles)
                            .expect("Failed to load character.");

                    world.add_resource(EffectReturn(character_handle));
                })
                .with_assertion(|world| {
                    let character_handle =
                        &world.read_resource::<EffectReturn<CharacterHandle>>().0;
                    let store = world.read_resource::<AssetStorage<Character>>();
                    assert!(store.get(character_handle).is_some());
                })
                .run()
                .is_ok()
        );
    }
}
