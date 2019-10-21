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
        CHAR_BAT_PATH, CHAR_BAT_SLUG, ENERGY_SQUARE_PATH, ENERGY_SQUARE_SLUG, MAP_FADE_PATH,
        MAP_FADE_SLUG,
    };
    use character_model::config::CharacterDefinition;
    use energy_model::config::EnergyDefinition;
    use loading_model::loaded::LoadStage;
    use map_model::config::MapDefinition;
    use object_type::ObjectType;
    use slotmap::SecondaryMap;

    use loading::{
        AssetDefinitionLoader, AssetDefinitionLoadingSystem, AssetLoadingResources,
        AssetPartLoader, DefinitionLoadingResources,
    };

    #[test]
    fn loads_character_definition() -> Result<(), Error> {
        run_test(
            SetupParams {
                asset_slug: CHAR_BAT_SLUG.clone(),
                asset_path: CHAR_BAT_PATH.clone(),
                asset_type: AssetType::Object(ObjectType::Character),
            },
            ExpectedParams {
                fn_assertion: |definition_loading_resources, asset_id| {
                    let DefinitionLoadingResources {
                        character_definition_assets,
                        asset_character_definition_handle,
                        ..
                    } = definition_loading_resources;

                    let character_definition_handle =
                        asset_character_definition_handle.get(asset_id);

                    assert!(character_definition_handle.is_some());

                    let character_definition_handle = character_definition_handle
                        .expect("Expected `CharacterDefinitionHandle` to exist.");
                    let character_definition =
                        character_definition_assets.get(character_definition_handle);

                    assert!(character_definition.is_some());
                },
            },
        )
    }

    #[test]
    fn loads_energy_definition() -> Result<(), Error> {
        run_test(
            SetupParams {
                asset_slug: ENERGY_SQUARE_SLUG.clone(),
                asset_path: ENERGY_SQUARE_PATH.clone(),
                asset_type: AssetType::Object(ObjectType::Energy),
            },
            ExpectedParams {
                fn_assertion: |definition_loading_resources, asset_id| {
                    let DefinitionLoadingResources {
                        energy_definition_assets,
                        asset_energy_definition_handle,
                        ..
                    } = definition_loading_resources;

                    let energy_definition_handle = asset_energy_definition_handle.get(asset_id);

                    assert!(energy_definition_handle.is_some());

                    let energy_definition_handle = energy_definition_handle
                        .expect("Expected `EnergyDefinitionHandle` to exist.");
                    let energy_definition = energy_definition_assets.get(energy_definition_handle);

                    assert!(energy_definition.is_some());
                },
            },
        )
    }

    #[test]
    fn loads_map_definition() -> Result<(), Error> {
        run_test(
            SetupParams {
                asset_slug: MAP_FADE_SLUG.clone(),
                asset_path: MAP_FADE_PATH.clone(),
                asset_type: AssetType::Map,
            },
            ExpectedParams {
                fn_assertion: |definition_loading_resources, asset_id| {
                    let DefinitionLoadingResources {
                        map_definition_assets,
                        asset_map_definition_handle,
                        ..
                    } = definition_loading_resources;

                    let map_definition_handle = asset_map_definition_handle.get(asset_id);

                    assert!(map_definition_handle.is_some());

                    let map_definition_handle =
                        map_definition_handle.expect("Expected `MapDefinitionHandle` to exist.");
                    let map_definition = map_definition_assets.get(map_definition_handle);

                    assert!(map_definition.is_some());
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
        ExpectedParams { fn_assertion }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(<AssetDefinitionLoadingSystem as System<'_>>::SystemData::setup)
            .with_system(Processor::<CharacterDefinition>::new(), "", &[])
            .with_system(Processor::<EnergyDefinition>::new(), "", &[])
            .with_system(Processor::<MapDefinition>::new(), "", &[])
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
            .with_assertion(|world| {
                // Assert that `is_complete` returns false before loading.
                let asset_id = *world.read_resource::<AssetId>();
                let (mut asset_loading_resources, mut definition_loading_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                assert!(!AssetDefinitionLoader::is_complete(
                    &mut asset_loading_resources,
                    &mut definition_loading_resources,
                    asset_id,
                ))
            })
            .with_effect(|world| {
                let asset_id = *world.read_resource::<AssetId>();
                let (mut asset_loading_resources, mut definition_loading_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                AssetDefinitionLoader::process(
                    &mut asset_loading_resources,
                    &mut definition_loading_resources,
                    asset_id,
                );
            })
            .with_state(|| {
                WaitForLoad::new_with_fn(|world| {
                    let load_stage_progress_counters =
                        world.read_resource::<HashMap<LoadStage, ProgressCounter>>();
                    load_stage_progress_counters
                        .get(&LoadStage::AssetDefinitionLoading)
                        .map(|progress_counter| progress_counter.is_complete())
                        .unwrap_or(false)
                })
            })
            .with_assertion(move |world| {
                let asset_id = *world.read_resource::<AssetId>();
                {
                    let definition_loading_resources =
                        world.system_data::<DefinitionLoadingResources<'_>>();

                    fn_assertion(&definition_loading_resources, asset_id);
                }

                let (mut asset_loading_resources, mut definition_loading_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                assert!(AssetDefinitionLoader::is_complete(
                    &mut asset_loading_resources,
                    &mut definition_loading_resources,
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
        fn_assertion: fn(&DefinitionLoadingResources<'_>, AssetId),
    }

    type TestSystemData<'s> = (
        Write<'s, SecondaryMap<AssetId, PathBuf>>,
        Write<'s, AssetIdMappings>,
        Write<'s, AssetTypeMappings>,
    );
    type AssetPartLoaderSystemData<'s> =
        (AssetLoadingResources<'s>, DefinitionLoadingResources<'s>);
}
