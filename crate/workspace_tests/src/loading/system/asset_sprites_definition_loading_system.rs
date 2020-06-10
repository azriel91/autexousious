#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use amethyst::{
        assets::{Processor, ProgressCounter},
        ecs::{System, SystemData, WorldExt, Write},
        Error,
    };
    use amethyst_test::{AmethystApplication, WaitForLoad};
    use asset_model::{
        config::{AssetSlug, AssetType},
        loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
    };
    use assets_test::{
        CHAR_BAT_PATH, CHAR_BAT_SLUG, MAP_EMPTY_PATH, MAP_EMPTY_SLUG, MAP_FADE_PATH, MAP_FADE_SLUG,
    };
    use loading_model::loaded::LoadStage;
    use loading_spi::{AssetLoadingResources, SpritesDefinitionLoadingResources};
    use object_type::ObjectType;
    use slotmap::SecondaryMap;
    use sprite_model::config::SpritesDefinition;

    use loading::{
        AssetPartLoader, AssetSpritesDefinitionLoader, AssetSpritesDefinitionLoadingSystem,
    };

    #[test]
    fn loads_sprites_definition_for_objects() -> Result<(), Error> {
        run_test(
            SetupParams {
                asset_slug: CHAR_BAT_SLUG.clone(),
                asset_path: CHAR_BAT_PATH.clone(),
                asset_type: AssetType::Object(ObjectType::Character),
            },
            ExpectedParams {
                is_complete_pre_load: false,
                fn_assertion: |sprites_definition_loading_resources, asset_id| {
                    let SpritesDefinitionLoadingResources {
                        sprites_definition_assets,
                        asset_sprites_definition_handles,
                    } = sprites_definition_loading_resources;

                    let sprites_definition_handle = asset_sprites_definition_handles.get(asset_id);

                    assert!(sprites_definition_handle.is_some());

                    let sprites_definition_handle = sprites_definition_handle
                        .expect("Expected `SpriteDefinitionHandle` to exist.");
                    let sprites_definition =
                        sprites_definition_assets.get(sprites_definition_handle);

                    assert!(sprites_definition.is_some());
                },
            },
        )
    }

    #[test]
    fn loads_sprites_definition_for_maps_with_sprites() -> Result<(), Error> {
        run_test(
            SetupParams {
                asset_slug: MAP_FADE_SLUG.clone(),
                asset_path: MAP_FADE_PATH.clone(),
                asset_type: AssetType::Map,
            },
            ExpectedParams {
                is_complete_pre_load: false,
                fn_assertion: |sprites_definition_loading_resources, asset_id| {
                    let SpritesDefinitionLoadingResources {
                        sprites_definition_assets,
                        asset_sprites_definition_handles,
                    } = sprites_definition_loading_resources;

                    let sprites_definition_handle = asset_sprites_definition_handles.get(asset_id);

                    assert!(sprites_definition_handle.is_some());

                    let sprites_definition_handle = sprites_definition_handle
                        .expect("Expected `SpriteDefinitionHandle` to exist.");
                    let sprites_definition =
                        sprites_definition_assets.get(sprites_definition_handle);

                    assert!(sprites_definition.is_some());
                },
            },
        )
    }

    #[test]
    fn skips_sprites_definition_for_maps_without_sprites() -> Result<(), Error> {
        run_test(
            SetupParams {
                asset_slug: MAP_EMPTY_SLUG.clone(),
                asset_path: MAP_EMPTY_PATH.clone(),
                asset_type: AssetType::Map,
            },
            ExpectedParams {
                is_complete_pre_load: true,
                fn_assertion: |sprites_definition_loading_resources, asset_id| {
                    let SpritesDefinitionLoadingResources {
                        asset_sprites_definition_handles,
                        ..
                    } = sprites_definition_loading_resources;

                    let sprites_definition_handle = asset_sprites_definition_handles.get(asset_id);

                    assert!(sprites_definition_handle.is_none());
                },
            },
        )
    }

    fn run_test(
        SetupParams {
            asset_slug,
            asset_path,
            asset_type,
        }: SetupParams,
        ExpectedParams {
            is_complete_pre_load,
            fn_assertion,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(<AssetSpritesDefinitionLoadingSystem as System<'_>>::SystemData::setup)
            .with_system(Processor::<SpritesDefinition>::new(), "", &[])
            .with_effect(move |world| {
                let asset_id = {
                    let (mut asset_id_to_path, mut asset_id_mappings, mut asset_type_mappings) =
                        world.system_data::<TestSystemData>();

                    let asset_id = asset_id_mappings.insert(asset_slug);
                    asset_id_to_path.insert(asset_id, asset_path);
                    asset_type_mappings.insert(asset_id, asset_type);

                    asset_id
                };

                world.insert(asset_id);
            })
            .with_assertion(move |world| {
                let asset_id = *world.read_resource::<AssetId>();
                let (mut asset_loading_resources, mut sprites_definition_loading_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                if is_complete_pre_load {
                    // Assert that `is_complete` returns true before loading. This may be true for
                    // maps.
                    assert!(AssetSpritesDefinitionLoader::is_complete(
                        &mut asset_loading_resources,
                        &mut sprites_definition_loading_resources,
                        asset_id,
                    ))
                } else {
                    // Assert that `is_complete` returns false before loading.
                    assert!(!AssetSpritesDefinitionLoader::is_complete(
                        &mut asset_loading_resources,
                        &mut sprites_definition_loading_resources,
                        asset_id,
                    ))
                }
            })
            .with_effect(|world| {
                let asset_id = *world.read_resource::<AssetId>();
                let (mut asset_loading_resources, mut sprites_definition_loading_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                AssetSpritesDefinitionLoader::process(
                    &mut asset_loading_resources,
                    &mut sprites_definition_loading_resources,
                    asset_id,
                );
            })
            .with_state(|| {
                WaitForLoad::new_with_fn(|world| {
                    let load_stage_progress_counters =
                        world.read_resource::<HashMap<LoadStage, ProgressCounter>>();
                    load_stage_progress_counters
                        .get(&LoadStage::SpritesDefinitionLoading)
                        .map(|progress_counter| progress_counter.is_complete())
                        .unwrap_or(false)
                })
            })
            .with_assertion(move |world| {
                let asset_id = *world.read_resource::<AssetId>();
                {
                    let sprites_definition_loading_resources =
                        world.system_data::<SpritesDefinitionLoadingResources<'_>>();

                    fn_assertion(&sprites_definition_loading_resources, asset_id);
                }

                let (mut asset_loading_resources, mut sprites_definition_loading_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                assert!(AssetSpritesDefinitionLoader::is_complete(
                    &mut asset_loading_resources,
                    &mut sprites_definition_loading_resources,
                    asset_id,
                ))
            })
            .run()
    }

    struct SetupParams {
        asset_slug: AssetSlug,
        asset_path: PathBuf,
        asset_type: AssetType,
    }

    struct ExpectedParams {
        is_complete_pre_load: bool,
        fn_assertion: fn(&SpritesDefinitionLoadingResources<'_>, AssetId),
    }

    type TestSystemData<'s> = (
        Write<'s, SecondaryMap<AssetId, PathBuf>>,
        Write<'s, AssetIdMappings>,
        Write<'s, AssetTypeMappings>,
    );
    type AssetPartLoaderSystemData<'s> = (
        AssetLoadingResources<'s>,
        SpritesDefinitionLoadingResources<'s>,
    );
}
