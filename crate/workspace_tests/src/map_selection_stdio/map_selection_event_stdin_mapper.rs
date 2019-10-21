#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{SystemData, World, WorldExt},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::config::AssetType;
    use assets_test::MAP_FADE_SLUG;
    use map_selection_model::{MapSelection, MapSelectionEvent};
    use stdio_spi::{StdinMapper, StdioError};

    use map_selection_stdio::{
        MapSelectionEventArgs, MapSelectionEventStdinMapper, MapSelectionEventStdinMapperSystemData,
    };

    #[test]
    fn returns_err_when_asset_slug_invalid() {
        let selection = "invalid".to_string();
        let args = MapSelectionEventArgs::Select { selection };
        let mut world = World::new();
        <MapSelectionEventStdinMapperSystemData<'_> as SystemData>::setup(&mut world);

        let result = MapSelectionEventStdinMapper::map(
            &world.system_data::<MapSelectionEventStdinMapperSystemData<'_>>(),
            args,
        );

        expect_err_msg(
            result,
            "Expected exactly one `/` in asset slug string: `invalid`.",
        );
    }

    #[test]
    fn returns_err_when_map_does_not_exist_for_slug() {
        let selection = "test/non_existent".to_string();
        let args = MapSelectionEventArgs::Select { selection };
        let mut world = World::new();
        <MapSelectionEventStdinMapperSystemData<'_> as SystemData>::setup(&mut world);

        let result = MapSelectionEventStdinMapper::map(
            &world.system_data::<MapSelectionEventStdinMapperSystemData<'_>>(),
            args,
        );

        expect_err_msg(result, "No map found with asset slug `test/non_existent`.");
    }

    #[test]
    fn maps_return_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_setup(|world| {
                <MapSelectionEventStdinMapperSystemData<'_> as SystemData>::setup(world)
            })
            .with_assertion(|world| {
                let args = MapSelectionEventArgs::Return;
                let map_selection_event_stdin_mapper_system_data =
                    world.system_data::<MapSelectionEventStdinMapperSystemData<'_>>();

                let result = MapSelectionEventStdinMapper::map(
                    &map_selection_event_stdin_mapper_system_data,
                    args,
                );

                assert!(result.is_ok());
                assert_eq!(MapSelectionEvent::Return, result.unwrap())
            })
            .run()
    }

    #[test]
    fn maps_deselect_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_setup(|world| {
                <MapSelectionEventStdinMapperSystemData<'_> as SystemData>::setup(world)
            })
            .with_assertion(|world| {
                let args = MapSelectionEventArgs::Deselect;
                let map_selection_event_stdin_mapper_system_data =
                    world.system_data::<MapSelectionEventStdinMapperSystemData<'_>>();

                let result = MapSelectionEventStdinMapper::map(
                    &map_selection_event_stdin_mapper_system_data,
                    args,
                );

                assert!(result.is_ok());
                assert_eq!(MapSelectionEvent::Deselect, result.unwrap())
            })
            .run()
    }

    #[test]
    fn maps_confirm_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_setup(|world| {
                <MapSelectionEventStdinMapperSystemData<'_> as SystemData>::setup(world)
            })
            .with_assertion(|world| {
                let args = MapSelectionEventArgs::Confirm;
                let map_selection_event_stdin_mapper_system_data =
                    world.system_data::<MapSelectionEventStdinMapperSystemData<'_>>();

                let result = MapSelectionEventStdinMapper::map(
                    &map_selection_event_stdin_mapper_system_data,
                    args,
                );

                assert!(result.is_ok());
                assert_eq!(MapSelectionEvent::Confirm, result.unwrap())
            })
            .run()
    }

    #[test]
    fn maps_select_id_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_setup(|world| {
                <MapSelectionEventStdinMapperSystemData<'_> as SystemData>::setup(world)
            })
            .with_assertion(|world| {
                let args = MapSelectionEventArgs::Select {
                    selection: MAP_FADE_SLUG.to_string(),
                };
                let map_selection_event_stdin_mapper_system_data =
                    world.system_data::<MapSelectionEventStdinMapperSystemData<'_>>();

                let result = MapSelectionEventStdinMapper::map(
                    &map_selection_event_stdin_mapper_system_data,
                    args,
                );

                assert!(result.is_ok());

                let asset_id = map_selection_event_stdin_mapper_system_data
                    .asset_id_mappings
                    .id(&*MAP_FADE_SLUG)
                    .copied()
                    .unwrap_or_else(|| panic!("Expected `{}` to be loaded.", &*MAP_FADE_SLUG));
                let map_selection = MapSelection::Id(asset_id);
                assert_eq!(MapSelectionEvent::Select { map_selection }, result.unwrap())
            })
            .run()
    }

    #[test]
    fn maps_select_random_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_setup(|world| {
                <MapSelectionEventStdinMapperSystemData<'_> as SystemData>::setup(world)
            })
            .with_assertion(|world| {
                let args = MapSelectionEventArgs::Select {
                    selection: "random".to_string(),
                };
                let map_selection_event_stdin_mapper_system_data =
                    world.system_data::<MapSelectionEventStdinMapperSystemData<'_>>();
                let asset_id = *map_selection_event_stdin_mapper_system_data
                    .asset_type_mappings
                    .iter_ids(&AssetType::Map)
                    .next()
                    .expect("Expected at least one map to be loaded.");

                let result = MapSelectionEventStdinMapper::map(
                    &map_selection_event_stdin_mapper_system_data,
                    args,
                );

                assert!(result.is_ok());
                let map_selection = MapSelection::Random(Some(asset_id));
                assert_eq!(MapSelectionEvent::Select { map_selection }, result.unwrap())
            })
            .run()
    }

    fn expect_err_msg(result: Result<MapSelectionEvent, Error>, expected: &str) {
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
