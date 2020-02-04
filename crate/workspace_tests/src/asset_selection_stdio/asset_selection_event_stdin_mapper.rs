#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Read, World, WorldExt},
        Error,
    };
    use application_test_support::{AssetQueries, AutexousiousApplication};
    use asset_model::loaded::AssetIdMappings;
    use asset_selection_model::{
        config::AssetSelectionEventArgs,
        play::{AssetSelection, AssetSelectionEvent},
    };
    use assets_test::CHAR_BAT_SLUG;
    use stdio_spi::{StdinMapper, StdioError};

    use asset_selection_stdio::AssetSelectionEventStdinMapper;

    macro_rules! test_map_direct {
        ($test_name:ident, $variant:ident) => {
            #[test]
            fn $test_name() {
                let args = AssetSelectionEventArgs::$variant;
                let mut world = World::empty();
                world.insert(AssetIdMappings::new());

                let result = AssetSelectionEventStdinMapper::map(
                    &Read::from(world.fetch::<AssetIdMappings>()),
                    args,
                );

                assert!(result.is_ok());
                assert_eq!(AssetSelectionEvent::$variant, result.unwrap())
            }
        };
    }

    macro_rules! test_map_with_controller_id {
        ($test_name:ident, $variant:ident) => {
            #[test]
            fn $test_name() {
                let controller_id = 0;
                let args = AssetSelectionEventArgs::$variant { controller_id };
                let mut world = World::empty();
                world.insert(AssetIdMappings::new());

                let result = AssetSelectionEventStdinMapper::map(
                    &Read::from(world.fetch::<AssetIdMappings>()),
                    args,
                );

                assert!(result.is_ok());
                assert_eq!(
                    AssetSelectionEvent::$variant {
                        entity: None,
                        controller_id
                    },
                    result.unwrap()
                )
            }
        };
    }

    macro_rules! test_map_with_slug {
        ($test_name:ident, $variant:ident, $slug_str:expr, $asset_selection_fn:expr) => {
            #[test]
            fn $test_name() -> Result<(), Error> {
                AutexousiousApplication::config_base()
                    .with_assertion(|world| {
                        let controller_id = 1;
                        let args = AssetSelectionEventArgs::$variant {
                            controller_id,
                            selection: $slug_str,
                        };
                        let asset_id_mappings = world.read_resource::<AssetIdMappings>();

                        let result = AssetSelectionEventStdinMapper::map(
                            &Read::from(asset_id_mappings),
                            args,
                        );

                        assert!(result.is_ok());

                        let asset_selection = $asset_selection_fn(&*world);
                        assert_eq!(
                            AssetSelectionEvent::$variant {
                                entity: None,
                                controller_id,
                                asset_selection
                            },
                            result.unwrap()
                        )
                    })
                    .run_isolated()
            }
        };
    }

    macro_rules! test_slug_invalid {
        ($test_name:ident, $variant:ident) => {
            #[test]
            fn $test_name() {
                let controller_id = 0;
                let selection = "invalid".to_string();
                let args = AssetSelectionEventArgs::$variant {
                    controller_id,
                    selection,
                };
                let mut world = World::empty();
                world.insert(AssetIdMappings::new());

                let result = AssetSelectionEventStdinMapper::map(
                    &Read::from(world.fetch::<AssetIdMappings>()),
                    args,
                );

                expect_err_msg(
                    result,
                    "Expected exactly one `/` in asset slug string: `invalid`.",
                );
            }
        };
    }

    macro_rules! test_err_when_char_not_exist {
        ($test_name:ident, $variant:ident) => {
            #[test]
            fn $test_name() {
                let controller_id = 0;
                let selection = "test/non_existent".to_string();
                let args = AssetSelectionEventArgs::$variant {
                    controller_id,
                    selection,
                };
                let mut world = World::empty();
                world.insert(AssetIdMappings::new());

                let result = AssetSelectionEventStdinMapper::map(
                    &Read::from(world.fetch::<AssetIdMappings>()),
                    args,
                );

                expect_err_msg(
                    result,
                    "No character found with asset slug `test/non_existent`.",
                );
            }
        };
    }

    test_slug_invalid!(returns_err_when_asset_slug_invalid_switch, Switch);
    test_slug_invalid!(returns_err_when_asset_slug_invalid_select, Select);
    test_err_when_char_not_exist!(
        returns_err_when_character_does_not_exist_for_slug_switch,
        Switch
    );
    test_err_when_char_not_exist!(
        returns_err_when_character_does_not_exist_for_slug_select,
        Select
    );

    test_map_with_slug!(
        maps_select_id_event,
        Select,
        CHAR_BAT_SLUG.to_string(),
        |world| {
            let asset_id = AssetQueries::id(world, &*CHAR_BAT_SLUG);
            AssetSelection::Id(asset_id)
        }
    );
    test_map_with_slug!(
        maps_select_random_event,
        Select,
        String::from("random"),
        |_| AssetSelection::Random
    );
    test_map_with_slug!(
        maps_switch_id_event,
        Switch,
        CHAR_BAT_SLUG.to_string(),
        |world| {
            let asset_id = AssetQueries::id(world, &*CHAR_BAT_SLUG);
            AssetSelection::Id(asset_id)
        }
    );
    test_map_with_slug!(
        maps_switch_random_event,
        Switch,
        String::from("random"),
        |_| AssetSelection::Random
    );

    test_map_with_controller_id!(maps_join_event, Join);
    test_map_with_controller_id!(maps_leave_event, Leave);
    test_map_with_controller_id!(maps_deselect_event, Deselect);
    test_map_direct!(maps_return_event, Return);
    test_map_direct!(maps_confirm_event, Confirm);

    fn expect_err_msg(result: Result<AssetSelectionEvent, Error>, expected: &str) {
        assert!(result.is_err());
        if let Some(stdio_error) = result
            .unwrap_err()
            .as_error()
            .downcast_ref::<Box<StdioError>>()
        {
            assert_eq!(
                &Box::new(StdioError::Msg(expected.to_string())),
                stdio_error
            );
        } else {
            panic!("Expected `StdioError::Msg({:?})`.", expected); // kcov-ignore
        }
    }
}
