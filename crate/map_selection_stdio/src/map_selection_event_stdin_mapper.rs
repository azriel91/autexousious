use std::str::FromStr;

use amethyst::Error;
use game_model::{
    config::AssetSlug,
    loaded::{MapAssets, SlugAndHandle},
};
use map_selection_model::{MapSelection, MapSelectionEvent};
use stdio_spi::{StdinMapper, StdioError};
use typename_derive::TypeName;

use crate::MapSelectionEventArgs;

/// Builds a `MapSelectionEvent` from stdin tokens.
#[derive(Debug, TypeName)]
pub struct MapSelectionEventStdinMapper;

impl MapSelectionEventStdinMapper {
    fn map_select_event(
        map_assets: &MapAssets,
        selection: &str,
    ) -> Result<MapSelectionEvent, Error> {
        let map_selection = match selection {
            "random" => {
                let snh = SlugAndHandle::from(
                    map_assets
                        .iter()
                        .next()
                        .expect("Expected at least one map to be loaded."),
                );
                MapSelection::Random(snh)
            }
            slug_str => {
                let slug = AssetSlug::from_str(slug_str).map_err(StdioError::Msg)?;
                let handle = map_assets
                    .get(&slug)
                    .ok_or_else(|| format!("No map found with asset slug `{}`.", slug))
                    .map_err(StdioError::Msg)?
                    .clone();

                let snh = SlugAndHandle { slug, handle };
                MapSelection::Id(snh)
            }
        };

        let map_selection_event = MapSelectionEvent::Select { map_selection };

        Ok(map_selection_event)
    }
}

impl StdinMapper for MapSelectionEventStdinMapper {
    type Resource = MapAssets;
    type Event = MapSelectionEvent;
    type Args = MapSelectionEventArgs;

    fn map(map_assets: &MapAssets, args: Self::Args) -> Result<Self::Event, Error> {
        match args {
            MapSelectionEventArgs::Select { selection } => {
                Self::map_select_event(map_assets, &selection)
            }
        } // kcov-ignore
    }
}

#[cfg(test)]
mod tests {
    use amethyst::Error;
    use application_test_support::AutexousiousApplication;
    use assets_test::ASSETS_MAP_FADE_SLUG;
    use game_model::loaded::{MapAssets, SlugAndHandle};
    use map_selection_model::{MapSelection, MapSelectionEvent};
    use stdio_spi::{StdinMapper, StdioError};

    use super::MapSelectionEventStdinMapper;
    use crate::MapSelectionEventArgs;

    #[test]
    fn returns_err_when_asset_slug_invalid() {
        let selection = "invalid".to_string();
        let args = MapSelectionEventArgs::Select { selection };
        let map_assets = MapAssets::new();

        let result = MapSelectionEventStdinMapper::map(&map_assets, args);

        expect_err_msg(
            result,
            "Expected exactly one `/` in slug string: \"invalid\".",
        );
    }

    #[test]
    fn returns_err_when_map_does_not_exist_for_slug() {
        let selection = "test/non_existent".to_string();
        let args = MapSelectionEventArgs::Select { selection };
        let map_assets = MapAssets::new();

        let result = MapSelectionEventStdinMapper::map(&map_assets, args);

        expect_err_msg(result, "No map found with asset slug `test/non_existent`.");
    }

    #[test]
    fn maps_select_id_event() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base("maps_select_id_event", false)
                .with_assertion(|world| {
                    let args = MapSelectionEventArgs::Select {
                        selection: ASSETS_MAP_FADE_SLUG.to_string(),
                    };
                    let map_assets = world.read_resource::<MapAssets>();

                    let result = MapSelectionEventStdinMapper::map(&*map_assets, args);

                    assert!(result.is_ok());
                    let snh = SlugAndHandle::from((&*map_assets, ASSETS_MAP_FADE_SLUG.clone()));
                    let map_selection = MapSelection::Id(snh);
                    assert_eq!(MapSelectionEvent::Select { map_selection }, result.unwrap())
                })
                .run()
                .is_ok()
        );
    }

    #[test]
    fn maps_select_random_event() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base("maps_select_random_event", false)
                .with_assertion(|world| {
                    let args = MapSelectionEventArgs::Select {
                        selection: "random".to_string(),
                    };
                    let map_assets = world.read_resource::<MapAssets>();

                    let result = MapSelectionEventStdinMapper::map(&*map_assets, args);

                    assert!(result.is_ok());
                    let snh = SlugAndHandle::from(
                        map_assets
                            .iter()
                            .next()
                            .expect("Expected at least one map to be loaded."),
                    );
                    let map_selection = MapSelection::Random(snh);
                    assert_eq!(MapSelectionEvent::Select { map_selection }, result.unwrap())
                })
                .run()
                .is_ok()
        );
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
