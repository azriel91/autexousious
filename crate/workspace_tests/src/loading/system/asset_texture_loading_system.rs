#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use amethyst::{
        assets::{Loader, ProgressCounter},
        ecs::{ReadExpect, System, SystemData, WorldExt, Write},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::{AmethystApplication, WaitForLoad};
    use application_test_support::AssetQueries;
    use asset_model::{
        config::AssetType,
        loaded::{AssetId, AssetTypeMappings},
    };
    use loading_model::loaded::LoadStage;
    use object_type::ObjectType;
    use slotmap::SecondaryMap;
    use sprite_loading::SpriteLoadingBundle;
    use sprite_model::config::SpritesDefinition;
    use test_support::{load_yaml, source_dir};

    use loading::{
        AssetLoadingResources, AssetPartLoader, AssetTextureLoader, AssetTextureLoadingSystem,
        SpritesDefinitionLoadingResources, TextureLoadingResources,
    };

    #[test]
    fn loads_textures() -> Result<(), Error> {
        let sprites_definition = load_yaml!(
            "asset_texture_loading_system_sprites_definition.yaml",
            SpritesDefinition
        );

        run_test(
            SetupParams {
                asset_path: source_dir!(),
                asset_type: AssetType::Object(ObjectType::Energy),
                fn_insert_definition: Box::new(|loader, definition_loading_resources, asset_id| {
                    let SpritesDefinitionLoadingResources {
                        sprites_definition_assets,
                        asset_sprites_definition_handles,
                    } = definition_loading_resources;

                    let sprites_definition_handle =
                        loader.load_from_data(sprites_definition, (), sprites_definition_assets);
                    asset_sprites_definition_handles.insert(asset_id, sprites_definition_handle);
                }),
            },
            ExpectedParams {
                fn_assertion: |texture_loading_resources, asset_id| {
                    let TextureLoadingResources {
                        texture_assets,
                        sprite_sheet_assets,
                        asset_sprite_sheet_handles,
                        ..
                    } = texture_loading_resources;

                    let sprite_sheet_handles = asset_sprite_sheet_handles.get(asset_id);
                    assert!(sprite_sheet_handles.is_some());

                    let sprite_sheet_handles =
                        sprite_sheet_handles.expect("Expected `SpriteSheetHandles` to exist.");
                    sprite_sheet_handles.iter().for_each(|sprite_sheet_handle| {
                        let sprite_sheet = sprite_sheet_assets
                            .get(sprite_sheet_handle)
                            .expect("Expected `SpriteSheet` to exist.");
                        let texture = texture_assets.get(&sprite_sheet.texture);

                        assert!(texture.is_some());
                    });
                },
            },
        )
    }

    fn run_test(
        SetupParams {
            asset_path,
            asset_type,
            fn_insert_definition,
        }: SetupParams,
        ExpectedParams { fn_assertion }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle_event_fn(|event_loop| RenderEmptyBundle::<DefaultBackend>::new(event_loop))
            .with_setup(<AssetTextureLoadingSystem as System<'_>>::SystemData::setup)
            .with_effect(move |world| {
                let asset_id = AssetQueries::id_generate_any(world);

                {
                    let (
                        mut asset_id_to_path,
                        mut asset_type_mappings,
                        loader,
                        mut definition_loading_resources,
                    ) = world.system_data::<TestSystemData>();

                    asset_id_to_path.insert(asset_id, asset_path);
                    asset_type_mappings.insert(asset_id, asset_type);
                    fn_insert_definition(&loader, &mut definition_loading_resources, asset_id);
                }
                world.insert(asset_id);
            })
            .with_assertion(|world| {
                // Assert that `is_complete` returns true before loading.
                let asset_id = *world.read_resource::<AssetId>();

                let (mut asset_loading_resources, mut texture_loading_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                assert!(AssetTextureLoader::is_complete(
                    &mut asset_loading_resources,
                    &mut texture_loading_resources,
                    asset_id,
                ))
            })
            .with_effect(|world| {
                let asset_id = *world.read_resource::<AssetId>();

                let (mut asset_loading_resources, mut texture_loading_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                AssetTextureLoader::process(
                    &mut asset_loading_resources,
                    &mut texture_loading_resources,
                    asset_id,
                );
            })
            .with_state(|| {
                WaitForLoad::new_with_fn(|world| {
                    let load_stage_progress_counters =
                        world.read_resource::<HashMap<LoadStage, ProgressCounter>>();
                    load_stage_progress_counters
                        .get(&LoadStage::TextureLoading)
                        .map(|progress_counter| progress_counter.is_complete())
                        .unwrap_or(false)
                })
            })
            .with_assertion(move |world| {
                let asset_id = *world.read_resource::<AssetId>();
                {
                    let texture_loading_resources =
                        world.system_data::<TextureLoadingResources<'_>>();
                    fn_assertion(&texture_loading_resources, asset_id);
                }

                let (mut asset_loading_resources, mut texture_loading_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                assert!(AssetTextureLoader::is_complete(
                    &mut asset_loading_resources,
                    &mut texture_loading_resources,
                    asset_id,
                ))
            })
            .run_isolated()
    }

    struct SetupParams {
        asset_path: PathBuf,
        asset_type: AssetType,
        fn_insert_definition: Box<
            dyn FnOnce(&Loader, &mut SpritesDefinitionLoadingResources<'_>, AssetId)
                + Send
                + Sync
                + 'static,
        >,
    }

    struct ExpectedParams {
        fn_assertion: fn(&TextureLoadingResources<'_>, AssetId),
    }

    type TestSystemData<'s> = (
        Write<'s, SecondaryMap<AssetId, PathBuf>>,
        Write<'s, AssetTypeMappings>,
        ReadExpect<'s, Loader>,
        SpritesDefinitionLoadingResources<'s>,
    );
    type AssetPartLoaderSystemData<'s> = (AssetLoadingResources<'s>, TextureLoadingResources<'s>);
}
