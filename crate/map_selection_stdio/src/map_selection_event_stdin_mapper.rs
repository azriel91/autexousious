use std::str::FromStr;

use amethyst::{ecs::Read, Error};
use asset_model::{
    config::AssetType,
    loaded::{AssetId, AssetIdMappings},
};
use map_selection_model::{MapSelection, MapSelectionEvent};
use stdio_spi::{MapperSystemData, StdinMapper, StdioError};
use typename_derive::TypeName;

use crate::MapSelectionEventArgs;

#[derive(Debug)]
pub struct MapSelectionEventStdinMapperData;

impl<'s> MapperSystemData<'s> for MapSelectionEventStdinMapperData {
    type SystemData = Read<'s, AssetIdMappings>;
}

/// Builds a `MapSelectionEvent` from stdin tokens.
#[derive(Debug, TypeName)]
pub struct MapSelectionEventStdinMapper;

impl MapSelectionEventStdinMapper {
    fn map_select_event(
        asset_id_mappings: &AssetIdMappings,
        selection: &str,
    ) -> Result<MapSelectionEvent, Error> {
        let map_selection = match selection {
            "random" => {
                let map_asset_id = SlugAndHandle::from(
                    asset_id_mappings
                        .iter_ids(AssetType::Map)
                        .next()
                        .expect("Expected at least one map to be loaded."),
                );
                MapSelection::Random(map_asset_id)
            }
            slug_str => {
                let slug = AssetSlug::from_str(slug_str)
                    .map_err(String::from)
                    .map_err(StdioError::Msg)?;
                let map_asset_id = asset_id_mappings
                    .slug(&slug)
                    .copied()
                    .ok_or_else(|| format!("No map found with asset slug `{}`.", slug))
                    .map_err(StdioError::Msg)?;

                MapSelection::Id(map_asset_id)
            }
        };

        let map_selection_event = MapSelectionEvent::Select { map_selection };

        Ok(map_selection_event)
    }
}

impl StdinMapper for MapSelectionEventStdinMapper {
    type SystemData = MapSelectionEventStdinMapperData;
    type Event = MapSelectionEvent;
    type Args = MapSelectionEventArgs;

    fn map(
        asset_id_mappings: &Read<'_, AssetIdMappings>,
        args: Self::Args,
    ) -> Result<Self::Event, Error> {
        match args {
            MapSelectionEventArgs::Return => Ok(MapSelectionEvent::Return),
            MapSelectionEventArgs::Select { selection } => {
                Self::map_select_event(asset_id_mappings, &selection)
            }
            MapSelectionEventArgs::Deselect => Ok(MapSelectionEvent::Deselect),
            MapSelectionEventArgs::Confirm => Ok(MapSelectionEvent::Confirm),
        } // kcov-ignore
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Read, World, WorldExt},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::loaded::SlugAndHandle;
    use assets_test::MAP_FADE_SLUG;
    use game_model::loaded::MapPrefabs;
    use map_selection_model::{MapSelection, MapSelectionEvent};
    use stdio_spi::{StdinMapper, StdioError};

    use super::MapSelectionEventStdinMapper;
    use crate::MapSelectionEventArgs;

    #[test]
    fn returns_err_when_asset_slug_invalid() {
        let selection = "invalid".to_string();
        let args = MapSelectionEventArgs::Select { selection };
        let mut world = World::new();
        world.insert(MapPrefabs::new());

        let result =
            MapSelectionEventStdinMapper::map(&Read::from(world.fetch::<MapPrefabs>()), args);

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
        world.insert(MapPrefabs::new());

        let result =
            MapSelectionEventStdinMapper::map(&Read::from(world.fetch::<MapPrefabs>()), args);

        expect_err_msg(result, "No map found with asset slug `test/non_existent`.");
    }

    #[test]
    fn maps_return_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_assertion(|world| {
                let args = MapSelectionEventArgs::Return;
                let map_prefabs = world.read_resource::<MapPrefabs>();

                let result = MapSelectionEventStdinMapper::map(&Read::from(map_prefabs), args);

                assert!(result.is_ok());
                assert_eq!(MapSelectionEvent::Return, result.unwrap())
            })
            .run_isolated()
    }

    #[test]
    fn maps_deselect_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_assertion(|world| {
                let args = MapSelectionEventArgs::Deselect;
                let map_prefabs = world.read_resource::<MapPrefabs>();

                let result = MapSelectionEventStdinMapper::map(&Read::from(map_prefabs), args);

                assert!(result.is_ok());
                assert_eq!(MapSelectionEvent::Deselect, result.unwrap())
            })
            .run_isolated()
    }

    #[test]
    fn maps_confirm_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_assertion(|world| {
                let args = MapSelectionEventArgs::Confirm;
                let map_prefabs = world.read_resource::<MapPrefabs>();

                let result = MapSelectionEventStdinMapper::map(&Read::from(map_prefabs), args);

                assert!(result.is_ok());
                assert_eq!(MapSelectionEvent::Confirm, result.unwrap())
            })
            .run_isolated()
    }

    #[test]
    fn maps_select_id_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_assertion(|world| {
                let args = MapSelectionEventArgs::Select {
                    selection: MAP_FADE_SLUG.to_string(),
                };
                let map_prefabs = world.read_resource::<MapPrefabs>();
                let snh = SlugAndHandle::from((&*map_prefabs, MAP_FADE_SLUG.clone()));

                let result = MapSelectionEventStdinMapper::map(&Read::from(map_prefabs), args);

                assert!(result.is_ok());
                let map_selection = MapSelection::Id(snh);
                assert_eq!(MapSelectionEvent::Select { map_selection }, result.unwrap())
            })
            .run_isolated()
    }

    #[test]
    fn maps_select_random_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_assertion(|world| {
                let args = MapSelectionEventArgs::Select {
                    selection: "random".to_string(),
                };
                let map_prefabs = world.read_resource::<MapPrefabs>();
                let snh = SlugAndHandle::from(
                    map_prefabs
                        .iter()
                        .next()
                        .expect("Expected at least one map to be loaded."),
                );

                let result = MapSelectionEventStdinMapper::map(&Read::from(map_prefabs), args);

                assert!(result.is_ok());
                let map_selection = MapSelection::Random(snh);
                assert_eq!(MapSelectionEvent::Select { map_selection }, result.unwrap())
            })
            .run_isolated()
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
