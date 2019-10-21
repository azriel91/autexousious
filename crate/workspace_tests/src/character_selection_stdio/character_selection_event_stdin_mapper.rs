#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Read, World, WorldExt},
        Error,
    };
    use application_test_support::{AssetQueries, AutexousiousApplication};
    use asset_model::loaded::AssetIdMappings;
    use assets_test::CHAR_BAT_SLUG;
    use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
    use stdio_spi::{StdinMapper, StdioError};

    use character_selection_stdio::{
        CharacterSelectionEventArgs, CharacterSelectionEventStdinMapper,
    };

    macro_rules! test_map_direct {
        ($test_name:ident, $variant:ident) => {
            #[test]
            fn $test_name() {
                let args = CharacterSelectionEventArgs::$variant;
                let mut world = World::empty();
                world.insert(AssetIdMappings::new());

                let result = CharacterSelectionEventStdinMapper::map(
                    &Read::from(world.fetch::<AssetIdMappings>()),
                    args,
                );

                assert!(result.is_ok());
                assert_eq!(CharacterSelectionEvent::$variant, result.unwrap())
            }
        };
    }

    macro_rules! test_map_with_controller_id {
        ($test_name:ident, $variant:ident) => {
            #[test]
            fn $test_name() {
                let controller_id = 0;
                let args = CharacterSelectionEventArgs::$variant { controller_id };
                let mut world = World::empty();
                world.insert(AssetIdMappings::new());

                let result = CharacterSelectionEventStdinMapper::map(
                    &Read::from(world.fetch::<AssetIdMappings>()),
                    args,
                );

                assert!(result.is_ok());
                assert_eq!(
                    CharacterSelectionEvent::$variant { controller_id },
                    result.unwrap()
                )
            }
        };
    }

    macro_rules! test_map_with_slug {
        ($test_name:ident, $variant:ident, $slug_str:expr, $character_selection_fn:expr) => {
            #[test]
            fn $test_name() -> Result<(), Error> {
                AutexousiousApplication::config_base()
                    .with_assertion(|world| {
                        let controller_id = 1;
                        let args = CharacterSelectionEventArgs::$variant {
                            controller_id,
                            selection: $slug_str,
                        };
                        let asset_id_mappings = world.read_resource::<AssetIdMappings>();

                        let result = CharacterSelectionEventStdinMapper::map(
                            &Read::from(asset_id_mappings),
                            args,
                        );

                        assert!(result.is_ok());

                        let character_selection = $character_selection_fn(&*world);
                        assert_eq!(
                            CharacterSelectionEvent::$variant {
                                controller_id,
                                character_selection
                            },
                            result.unwrap()
                        )
                    })
                    .run()
            }
        };
    }

    macro_rules! test_slug_invalid {
        ($test_name:ident, $variant:ident) => {
            #[test]
            fn $test_name() {
                let controller_id = 0;
                let selection = "invalid".to_string();
                let args = CharacterSelectionEventArgs::$variant {
                    controller_id,
                    selection,
                };
                let mut world = World::empty();
                world.insert(AssetIdMappings::new());

                let result = CharacterSelectionEventStdinMapper::map(
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
                let args = CharacterSelectionEventArgs::$variant {
                    controller_id,
                    selection,
                };
                let mut world = World::empty();
                world.insert(AssetIdMappings::new());

                let result = CharacterSelectionEventStdinMapper::map(
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
            CharacterSelection::Id(asset_id)
        }
    );
    test_map_with_slug!(
        maps_select_random_event,
        Select,
        String::from("random"),
        |_| CharacterSelection::Random
    );
    test_map_with_slug!(
        maps_switch_id_event,
        Switch,
        CHAR_BAT_SLUG.to_string(),
        |world| {
            let asset_id = AssetQueries::id(world, &*CHAR_BAT_SLUG);
            CharacterSelection::Id(asset_id)
        }
    );
    test_map_with_slug!(
        maps_switch_random_event,
        Switch,
        String::from("random"),
        |_| CharacterSelection::Random
    );

    test_map_with_controller_id!(maps_join_event, Join);
    test_map_with_controller_id!(maps_leave_event, Leave);
    test_map_with_controller_id!(maps_deselect_event, Deselect);
    test_map_direct!(maps_return_event, Return);
    test_map_direct!(maps_confirm_event, Confirm);

    fn expect_err_msg(result: Result<CharacterSelectionEvent, Error>, expected: &str) {
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
