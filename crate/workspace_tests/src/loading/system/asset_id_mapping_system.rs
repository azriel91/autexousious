#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use amethyst::{
        assets::Loader,
        ecs::{ReadExpect, System, SystemData, World, WorldExt, Write},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{
        config::{AssetSlug, AssetType},
        loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
    };
    use assets_test::{CHAR_BAT_SLUG, ENERGY_SQUARE_SLUG, UI_CHARACTER_SELECTION_SLUG};
    use background_loading::BackgroundLoadingBundle;
    use background_model::config::BackgroundDefinition;
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
    use ui_loading::UiLoadingBundle;
    use ui_model::config::UiDefinition;
    use ui_model_spi::config::UiSequenceName;

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
                asset_paramses: vec![AssetParams {
                    asset_slug: CHAR_BAT_SLUG.clone(),
                    asset_type: AssetType::Object(ObjectType::Character),
                    fn_insert_definition: Box::new(
                        |loader, definition_loading_resources, asset_id| {
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
                            asset_character_definition_handle
                                .insert(asset_id, character_definition_handle);
                        },
                    ),
                }],
            },
            ExpectedParams {
                fn_assertions: vec![assert_character_ids_mapped],
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
                asset_paramses: vec![AssetParams {
                    asset_slug: ENERGY_SQUARE_SLUG.clone(),
                    asset_type: AssetType::Object(ObjectType::Energy),
                    fn_insert_definition: Box::new(
                        |loader, definition_loading_resources, asset_id| {
                            let DefinitionLoadingResources {
                                energy_definition_assets,
                                asset_energy_definition_handle,
                                ..
                            } = definition_loading_resources;

                            let energy_definition_handle = loader.load_from_data(
                                energy_definition,
                                (),
                                energy_definition_assets,
                            );
                            asset_energy_definition_handle
                                .insert(asset_id, energy_definition_handle);
                        },
                    ),
                }],
            },
            ExpectedParams {
                fn_assertions: vec![assert_energy_ids_mapped],
            },
        )
    }

    #[test]
    fn waits_for_spawn_ids_to_be_mapped() -> Result<(), Error> {
        let spawner_definition =
            load_yaml!("asset_id_mapping_system_spawns.yaml", CharacterDefinition);
        let character_definition = load_yaml!(
            "asset_id_mapping_system_character_definition.yaml",
            CharacterDefinition
        );
        let energy_definition = load_yaml!(
            "asset_id_mapping_system_energy_definition.yaml",
            EnergyDefinition
        );

        run_test(
            SetupParams {
                asset_paramses: vec![
                    AssetParams {
                        asset_slug: AssetSlug::from_str("test/spawner")
                            .expect("Expected asset slug to be valid."),
                        asset_type: AssetType::Object(ObjectType::Character),
                        fn_insert_definition: Box::new(
                            |loader, definition_loading_resources, asset_id| {
                                let DefinitionLoadingResources {
                                    character_definition_assets,
                                    asset_character_definition_handle,
                                    ..
                                } = definition_loading_resources;

                                let character_definition_handle = loader.load_from_data(
                                    spawner_definition,
                                    (),
                                    character_definition_assets,
                                );
                                asset_character_definition_handle
                                    .insert(asset_id, character_definition_handle);
                            },
                        ),
                    },
                    AssetParams {
                        asset_slug: CHAR_BAT_SLUG.clone(),
                        asset_type: AssetType::Object(ObjectType::Character),
                        fn_insert_definition: Box::new(
                            |loader, definition_loading_resources, asset_id| {
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
                                asset_character_definition_handle
                                    .insert(asset_id, character_definition_handle);
                            },
                        ),
                    },
                    AssetParams {
                        asset_slug: ENERGY_SQUARE_SLUG.clone(),
                        asset_type: AssetType::Object(ObjectType::Energy),
                        fn_insert_definition: Box::new(
                            |loader, definition_loading_resources, asset_id| {
                                let DefinitionLoadingResources {
                                    energy_definition_assets,
                                    asset_energy_definition_handle,
                                    ..
                                } = definition_loading_resources;

                                let energy_definition_handle = loader.load_from_data(
                                    energy_definition,
                                    (),
                                    energy_definition_assets,
                                );
                                asset_energy_definition_handle
                                    .insert(asset_id, energy_definition_handle);
                            },
                        ),
                    },
                ],
            },
            ExpectedParams {
                fn_assertions: vec![
                    |id_mapping_resources, asset_id| {
                        let IdMappingResources {
                            asset_sequence_id_mappings_character,
                            ..
                        } = id_mapping_resources;

                        let sequence_id_mappings =
                            asset_sequence_id_mappings_character.get(asset_id);

                        assert!(sequence_id_mappings.is_some());

                        let sequence_id_mappings = sequence_id_mappings
                            .expect("Expected `SequenceIdMappings<Character>` to exist.");
                        let mut sequence_id_mappings_expected =
                            SequenceIdMappings::with_capacity(10);
                        sequence_id_mappings_expected.insert(
                            SequenceNameString::Name(CharacterSequenceName::Stand),
                            SequenceId(0),
                        );

                        assert_eq!(&sequence_id_mappings_expected, sequence_id_mappings);
                    },
                    assert_character_ids_mapped,
                    assert_energy_ids_mapped,
                ],
            },
        )
    }

    #[test]
    fn inserts_empty_ui_mappings() -> Result<(), Error> {
        run_test(
            SetupParams {
                asset_paramses: vec![AssetParams {
                    asset_slug: UI_CHARACTER_SELECTION_SLUG.clone(),
                    asset_type: AssetType::Ui,
                    fn_insert_definition: Box::new(|_, _, _| {}),
                }],
            },
            ExpectedParams {
                fn_assertions: vec![assert_no_ui_ids_mapped],
            },
        )
    }

    #[test]
    fn maps_background_ids() -> Result<(), Error> {
        let background_definition = load_yaml!(
            "asset_id_mapping_system_background_definition.yaml",
            BackgroundDefinition
        );

        run_test(
            SetupParams {
                asset_paramses: vec![AssetParams {
                    asset_slug: UI_CHARACTER_SELECTION_SLUG.clone(),
                    asset_type: AssetType::Ui,
                    fn_insert_definition: Box::new(
                        |loader, definition_loading_resources, asset_id| {
                            let DefinitionLoadingResources {
                                background_definition_assets,
                                asset_background_definition_handle,
                                ..
                            } = definition_loading_resources;

                            let background_definition_handle = loader.load_from_data(
                                background_definition,
                                (),
                                background_definition_assets,
                            );
                            asset_background_definition_handle
                                .insert(asset_id, background_definition_handle);
                        },
                    ),
                }],
            },
            ExpectedParams {
                fn_assertions: vec![assert_background_ids_mapped],
            },
        )
    }

    #[test]
    fn maps_ui_ids() -> Result<(), Error> {
        let ui_definition = load_yaml!("asset_id_mapping_system_ui_definition.yaml", UiDefinition);

        run_test(
            SetupParams {
                asset_paramses: vec![AssetParams {
                    asset_slug: UI_CHARACTER_SELECTION_SLUG.clone(),
                    asset_type: AssetType::Ui,
                    fn_insert_definition: Box::new(
                        |loader, definition_loading_resources, asset_id| {
                            let DefinitionLoadingResources {
                                ui_definition_assets,
                                asset_ui_definition_handle,
                                ..
                            } = definition_loading_resources;

                            let ui_definition_handle =
                                loader.load_from_data(ui_definition, (), ui_definition_assets);
                            asset_ui_definition_handle.insert(asset_id, ui_definition_handle);
                        },
                    ),
                }],
            },
            ExpectedParams {
                fn_assertions: vec![assert_ui_ids_mapped],
            },
        )
    }

    #[test]
    fn maps_background_and_ui_ids() -> Result<(), Error> {
        let background_definition = load_yaml!(
            "asset_id_mapping_system_background_definition.yaml",
            BackgroundDefinition
        );
        let ui_definition = load_yaml!("asset_id_mapping_system_ui_definition.yaml", UiDefinition);

        run_test(
            SetupParams {
                asset_paramses: vec![AssetParams {
                    asset_slug: UI_CHARACTER_SELECTION_SLUG.clone(),
                    asset_type: AssetType::Ui,
                    fn_insert_definition: Box::new(
                        |loader, definition_loading_resources, asset_id| {
                            let DefinitionLoadingResources {
                                background_definition_assets,
                                ui_definition_assets,
                                asset_background_definition_handle,
                                asset_ui_definition_handle,
                                ..
                            } = definition_loading_resources;

                            let background_definition_handle = loader.load_from_data(
                                background_definition,
                                (),
                                background_definition_assets,
                            );
                            asset_background_definition_handle
                                .insert(asset_id, background_definition_handle);

                            let ui_definition_handle =
                                loader.load_from_data(ui_definition, (), ui_definition_assets);
                            asset_ui_definition_handle.insert(asset_id, ui_definition_handle);
                        },
                    ),
                }],
            },
            ExpectedParams {
                fn_assertions: vec![assert_background_and_ui_ids_mapped],
            },
        )
    }

    fn run_test(
        SetupParams { asset_paramses }: SetupParams,
        ExpectedParams { fn_assertions }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(EnergyLoadingBundle::new())
            .with_bundle(BackgroundLoadingBundle::new())
            .with_bundle(UiLoadingBundle::new())
            .with_setup(<AssetIdMappingSystem as System<'_>>::SystemData::setup)
            .with_effect(move |world| {
                let asset_ids = asset_paramses
                    .into_iter()
                    .map(|asset_params| insert_asset_data(world, asset_params))
                    .collect::<Vec<AssetId>>();
                world.insert(asset_ids);
            })
            .with_assertion(|world| {
                // Assert that `is_complete` returns false before loading.
                let asset_ids = world.read_resource::<Vec<AssetId>>();

                asset_ids.iter().for_each(|asset_id| {
                    let (mut asset_loading_resources, mut id_mapping_resources) =
                        world.system_data::<AssetPartLoaderSystemData<'_>>();

                    assert!(!AssetIdMapper::is_complete(
                        &mut asset_loading_resources,
                        &mut id_mapping_resources,
                        *asset_id,
                    ))
                })
            })
            .with_effect(|world| {
                let asset_ids = world.read_resource::<Vec<AssetId>>();

                asset_ids.iter().for_each(|asset_id| {
                    let (mut asset_loading_resources, mut id_mapping_resources) =
                        world.system_data::<AssetPartLoaderSystemData<'_>>();

                    AssetIdMapper::process(
                        &mut asset_loading_resources,
                        &mut id_mapping_resources,
                        *asset_id,
                    );
                })
            })
            .with_assertion(move |world| {
                let asset_ids = world.read_resource::<Vec<AssetId>>();
                {
                    let id_mapping_resources = world.system_data::<IdMappingResources<'_>>();

                    fn_assertions.iter().zip(asset_ids.iter()).for_each(
                        |(fn_assertion, asset_id)| {
                            fn_assertion(&id_mapping_resources, *asset_id);
                        },
                    )
                }

                let (mut asset_loading_resources, mut id_mapping_resources) =
                    world.system_data::<AssetPartLoaderSystemData<'_>>();

                asset_ids.iter().for_each(|asset_id| {
                    assert!(AssetIdMapper::is_complete(
                        &mut asset_loading_resources,
                        &mut id_mapping_resources,
                        *asset_id,
                    ))
                })
            })
            .run()
    }

    fn insert_asset_data(
        world: &mut World,
        AssetParams {
            asset_slug,
            asset_type,
            fn_insert_definition,
        }: AssetParams,
    ) -> AssetId {
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
    }

    fn assert_character_ids_mapped(
        id_mapping_resources: &IdMappingResources<'_>,
        asset_id: AssetId,
    ) {
        let IdMappingResources {
            asset_sequence_id_mappings_character,
            ..
        } = id_mapping_resources;

        let sequence_id_mappings = asset_sequence_id_mappings_character.get(asset_id);

        assert!(sequence_id_mappings.is_some());

        let sequence_id_mappings = sequence_id_mappings
            .expect("Expected `SequenceIdMappings<CharacterSequenceName>` to exist.");
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
    }

    fn assert_energy_ids_mapped(id_mapping_resources: &IdMappingResources<'_>, asset_id: AssetId) {
        let IdMappingResources {
            asset_sequence_id_mappings_energy,
            ..
        } = id_mapping_resources;

        let sequence_id_mappings = asset_sequence_id_mappings_energy.get(asset_id);

        assert!(sequence_id_mappings.is_some());

        let sequence_id_mappings = sequence_id_mappings
            .expect("Expected `SequenceIdMappings<EnergySequenceName>` to exist.");
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
    }

    fn assert_no_ui_ids_mapped(id_mapping_resources: &IdMappingResources<'_>, asset_id: AssetId) {
        let IdMappingResources {
            asset_sequence_id_mappings_ui,
            ..
        } = id_mapping_resources;

        let sequence_id_mappings = asset_sequence_id_mappings_ui.get(asset_id);

        assert!(sequence_id_mappings.is_some());

        let sequence_id_mappings =
            sequence_id_mappings.expect("Expected `SequenceIdMappings<UiSequenceName>` to exist.");
        let sequence_id_mappings_expected = SequenceIdMappings::with_capacity(10);

        assert_eq!(&sequence_id_mappings_expected, sequence_id_mappings);
    }

    fn assert_background_ids_mapped(
        id_mapping_resources: &IdMappingResources<'_>,
        asset_id: AssetId,
    ) {
        let IdMappingResources {
            asset_sequence_id_mappings_ui,
            ..
        } = id_mapping_resources;

        let sequence_id_mappings = asset_sequence_id_mappings_ui.get(asset_id);

        assert!(sequence_id_mappings.is_some());

        let sequence_id_mappings =
            sequence_id_mappings.expect("Expected `SequenceIdMappings<UiSequenceName>` to exist.");
        let mut sequence_id_mappings_expected = SequenceIdMappings::with_capacity(10);
        sequence_id_mappings_expected.insert(
            SequenceNameString::String(String::from("background_layer_0")),
            SequenceId(0),
        );
        sequence_id_mappings_expected.insert(
            SequenceNameString::String(String::from("background_layer_1")),
            SequenceId(1),
        );

        assert_eq!(&sequence_id_mappings_expected, sequence_id_mappings);
    }

    fn assert_ui_ids_mapped(id_mapping_resources: &IdMappingResources<'_>, asset_id: AssetId) {
        let IdMappingResources {
            asset_sequence_id_mappings_ui,
            ..
        } = id_mapping_resources;

        let sequence_id_mappings = asset_sequence_id_mappings_ui.get(asset_id);

        assert!(sequence_id_mappings.is_some());

        let sequence_id_mappings =
            sequence_id_mappings.expect("Expected `SequenceIdMappings<UiSequenceName>` to exist.");
        let mut sequence_id_mappings_expected = SequenceIdMappings::with_capacity(10);
        sequence_id_mappings_expected.insert(
            SequenceNameString::String(String::from("start_game_inactive")),
            SequenceId(0),
        );
        sequence_id_mappings_expected.insert(
            SequenceNameString::Name(UiSequenceName::Active),
            SequenceId(1),
        );
        sequence_id_mappings_expected.insert(
            SequenceNameString::String(String::from("start_game_selected")),
            SequenceId(2),
        );

        assert_eq!(&sequence_id_mappings_expected, sequence_id_mappings);
    }

    fn assert_background_and_ui_ids_mapped(
        id_mapping_resources: &IdMappingResources<'_>,
        asset_id: AssetId,
    ) {
        let IdMappingResources {
            asset_sequence_id_mappings_ui,
            ..
        } = id_mapping_resources;

        let sequence_id_mappings = asset_sequence_id_mappings_ui.get(asset_id);

        assert!(sequence_id_mappings.is_some());

        let sequence_id_mappings =
            sequence_id_mappings.expect("Expected `SequenceIdMappings<UiSequenceName>` to exist.");
        let mut sequence_id_mappings_expected = SequenceIdMappings::with_capacity(10);
        sequence_id_mappings_expected.insert(
            SequenceNameString::String(String::from("background_layer_0")),
            SequenceId(0),
        );
        sequence_id_mappings_expected.insert(
            SequenceNameString::String(String::from("background_layer_1")),
            SequenceId(1),
        );
        sequence_id_mappings_expected.insert(
            SequenceNameString::String(String::from("start_game_inactive")),
            SequenceId(2),
        );
        sequence_id_mappings_expected.insert(
            SequenceNameString::Name(UiSequenceName::Active),
            SequenceId(3),
        );
        sequence_id_mappings_expected.insert(
            SequenceNameString::String(String::from("start_game_selected")),
            SequenceId(4),
        );

        assert_eq!(&sequence_id_mappings_expected, sequence_id_mappings);
    }

    struct SetupParams {
        asset_paramses: Vec<AssetParams>,
    }

    struct AssetParams {
        asset_slug: AssetSlug,
        asset_type: AssetType,
        fn_insert_definition: Box<
            dyn FnOnce(&Loader, &mut DefinitionLoadingResources<'_>, AssetId)
                + Send
                + Sync
                + 'static,
        >,
    }

    struct ExpectedParams {
        fn_assertions: Vec<fn(&IdMappingResources<'_>, AssetId)>,
    }

    type TestSystemData<'s> = (
        Write<'s, AssetIdMappings>,
        Write<'s, AssetTypeMappings>,
        ReadExpect<'s, Loader>,
        DefinitionLoadingResources<'s>,
    );
    type AssetPartLoaderSystemData<'s> = (AssetLoadingResources<'s>, IdMappingResources<'s>);
}
