#[cfg(test)]
mod tests {
    use amethyst::{
        assets::Loader,
        ecs::{ReadExpect, System, SystemData, WorldExt, Write},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{
        config::{AssetSlug, AssetType},
        loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
    };
    use assets_test::{CHAR_BAT_SLUG, ENERGY_SQUARE_SLUG};
    use character_loading::CharacterLoadingBundle;
    use character_model::config::{CharacterDefinition, CharacterSequenceName};
    use energy_loading::EnergyLoadingBundle;
    use energy_model::config::{EnergyDefinition, EnergySequenceName};
    use object_type::ObjectType;
    use sequence_model::{
        config::SequenceNameString,
        loaded::{SequenceId, SequenceIdMappings},
    };
    use test_support::load_yaml;

    use loading::{
        AssetIdMapper, AssetIdMappingSystem, AssetLoadingResources, AssetPartLoader,
        DefinitionLoadingResources, IdMappingResources,
    };

    #[test]
    fn preprocess_sets_asset_sequence_id_mappings_capacity() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(<AssetIdMappingSystem as System<'_>>::SystemData::setup)
            .with_assertion(|world| {
                // Assert that capacity for `asset_sequence_id_mappings_*` is not set before.
                let IdMappingResources {
                    asset_sequence_id_mappings_character,
                    asset_sequence_id_mappings_energy,
                    ..
                } = world.system_data::<IdMappingResources<'_>>();

                assert_eq!(0, asset_sequence_id_mappings_character.capacity());
                assert_eq!(0, asset_sequence_id_mappings_energy.capacity());
            })
            .with_effect(|world| {
                {
                    let mut asset_id_mappings = world.write_resource::<AssetIdMappings>();
                    asset_id_mappings.reserve(10);
                }

                let (mut asset_loading_resources, mut id_mapping_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                AssetIdMapper::preprocess(&mut asset_loading_resources, &mut id_mapping_resources);
            })
            .with_assertion(|world| {
                // Assert that capacity for `asset_sequence_id_mappings_*` is set after.
                let IdMappingResources {
                    asset_sequence_id_mappings_character,
                    asset_sequence_id_mappings_energy,
                    ..
                } = world.system_data::<IdMappingResources<'_>>();

                assert_eq!(10, asset_sequence_id_mappings_character.capacity());
                assert_eq!(10, asset_sequence_id_mappings_energy.capacity());
            })
            .run()
    }

    #[test]
    fn maps_character_ids() -> Result<(), Error> {
        let character_definition = load_yaml!(
            "asset_id_mapping_system_character_definition.yaml",
            CharacterDefinition
        );

        run_test(
            SetupParams {
                asset_slug: CHAR_BAT_SLUG.clone(),
                asset_type: AssetType::Object(ObjectType::Character),
                fn_insert_definition: |loader, definition_loading_resources, asset_id| {
                    let DefinitionLoadingResources {
                        character_definition_assets,
                        asset_character_definition_handle,
                        ..
                    } = definition_loading_resources;

                    let character_definition_handle = loader.load_from_data(
                        character_definition,
                        (),
                        character_definition_assets,
                    );
                    asset_character_definition_handle.insert(asset_id, character_definition_handle);
                },
            },
            ExpectedParams {
                fn_assertion: |id_mapping_resources, asset_id| {
                    let IdMappingResources {
                        asset_sequence_id_mappings_character,
                        ..
                    } = id_mapping_resources;

                    let sequence_id_mappings = asset_sequence_id_mappings_character.get(asset_id);

                    assert!(sequence_id_mappings.is_some());

                    let sequence_id_mappings = sequence_id_mappings
                        .expect("Expected `SequenceIdMappings<Character>` to exist.");
                    let mut sequence_id_mappings_expected = SequenceIdMappings::with_capacity(10);
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::Name(CharacterSequenceName::Stand),
                        SequenceId(0),
                    );
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::Name(CharacterSequenceName::StandAttack0),
                        SequenceId(1),
                    );
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::Name(CharacterSequenceName::StandAttack1),
                        SequenceId(2),
                    );
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::String(String::from("custom_string_a")),
                        SequenceId(3),
                    );
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::String(String::from("custom_string_b")),
                        SequenceId(4),
                    );
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::Name(CharacterSequenceName::Walk),
                        SequenceId(5),
                    );

                    assert_eq!(&sequence_id_mappings_expected, sequence_id_mappings);
                },
            },
        )
    }

    #[test]
    fn maps_energy_ids() -> Result<(), Error> {
        let energy_definition = load_yaml!(
            "asset_id_mapping_system_energy_definition.yaml",
            EnergyDefinition
        );

        run_test(
            SetupParams {
                asset_slug: ENERGY_SQUARE_SLUG.clone(),
                asset_type: AssetType::Object(ObjectType::Energy),
                fn_insert_definition: |loader, definition_loading_resources, asset_id| {
                    let DefinitionLoadingResources {
                        energy_definition_assets,
                        asset_energy_definition_handle,
                        ..
                    } = definition_loading_resources;

                    let energy_definition_handle =
                        loader.load_from_data(energy_definition, (), energy_definition_assets);
                    asset_energy_definition_handle.insert(asset_id, energy_definition_handle);
                },
            },
            ExpectedParams {
                fn_assertion: |id_mapping_resources, asset_id| {
                    let IdMappingResources {
                        asset_sequence_id_mappings_energy,
                        ..
                    } = id_mapping_resources;

                    let sequence_id_mappings = asset_sequence_id_mappings_energy.get(asset_id);

                    assert!(sequence_id_mappings.is_some());

                    let sequence_id_mappings = sequence_id_mappings
                        .expect("Expected `SequenceIdMappings<Energy>` to exist.");
                    let mut sequence_id_mappings_expected = SequenceIdMappings::with_capacity(10);
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::Name(EnergySequenceName::Hover),
                        SequenceId(0),
                    );
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::String(String::from("fly")),
                        SequenceId(1),
                    );
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::Name(EnergySequenceName::Hit),
                        SequenceId(2),
                    );
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::Name(EnergySequenceName::Hitting),
                        SequenceId(3),
                    );
                    sequence_id_mappings_expected.insert(
                        SequenceNameString::String(String::from("pow")),
                        SequenceId(4),
                    );

                    assert_eq!(&sequence_id_mappings_expected, sequence_id_mappings);
                },
            },
        )
    }

    fn run_test<F>(
        SetupParams {
            asset_slug,
            asset_type,
            fn_insert_definition,
        }: SetupParams<F>,
        ExpectedParams { fn_assertion }: ExpectedParams,
    ) -> Result<(), Error>
    where
        F: FnOnce(&Loader, &mut DefinitionLoadingResources<'_>, AssetId) + Send + Sync + 'static,
    {
        AmethystApplication::blank()
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(EnergyLoadingBundle::new())
            .with_setup(<AssetIdMappingSystem as System<'_>>::SystemData::setup)
            .with_effect(move |world| {
                let asset_id = {
                    let (
                        mut asset_id_mappings,
                        mut asset_type_mappings,
                        loader,
                        mut definition_loading_resources,
                    ) = world.system_data::<TestSystemData>();

                    let asset_id = asset_id_mappings.insert(asset_slug);
                    asset_type_mappings.insert(asset_id, asset_type);
                    fn_insert_definition(&loader, &mut definition_loading_resources, asset_id);

                    asset_id
                };

                world.insert(asset_id);
            })
            .with_assertion(|world| {
                // Assert that `is_complete` returns false before loading.
                let asset_id = *world.read_resource::<AssetId>();
                let (mut asset_loading_resources, mut id_mapping_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                assert!(!AssetIdMapper::is_complete(
                    &mut asset_loading_resources,
                    &mut id_mapping_resources,
                    asset_id,
                ))
            })
            .with_effect(|world| {
                let asset_id = *world.read_resource::<AssetId>();
                let (mut asset_loading_resources, mut id_mapping_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                AssetIdMapper::process(
                    &mut asset_loading_resources,
                    &mut id_mapping_resources,
                    asset_id,
                );
            })
            .with_assertion(move |world| {
                let asset_id = *world.read_resource::<AssetId>();
                {
                    let id_mapping_resources = world.system_data::<IdMappingResources<'_>>();

                    fn_assertion(&id_mapping_resources, asset_id);
                }

                let (mut asset_loading_resources, mut id_mapping_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                assert!(AssetIdMapper::is_complete(
                    &mut asset_loading_resources,
                    &mut id_mapping_resources,
                    asset_id,
                ))
            })
            .run()
    }

    struct SetupParams<F>
    where
        F: FnOnce(&Loader, &mut DefinitionLoadingResources<'_>, AssetId) + Send + Sync + 'static,
    {
        asset_slug: AssetSlug,
        asset_type: AssetType,
        fn_insert_definition: F,
    }

    struct ExpectedParams {
        fn_assertion: fn(&IdMappingResources<'_>, AssetId),
    }

    type TestSystemData<'s> = (
        Write<'s, AssetIdMappings>,
        Write<'s, AssetTypeMappings>,
        ReadExpect<'s, Loader>,
        DefinitionLoadingResources<'s>,
    );
    type AssetPartLoaderSystemData<'s> = (AssetLoadingResources<'s>, IdMappingResources<'s>);
}
