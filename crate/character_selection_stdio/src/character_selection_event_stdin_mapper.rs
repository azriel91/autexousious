use std::str::FromStr;

use amethyst::{ecs::Read, Error};
use asset_model::{
    config::AssetSlug,
    loaded::{CharacterAssets, SlugAndHandle},
};
use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
use game_input_model::ControllerId;
use stdio_spi::{MapperSystemData, StdinMapper, StdioError};
use typename_derive::TypeName;

use crate::CharacterSelectionEventArgs;

#[derive(Debug)]
pub struct CharacterSelectionEventStdinMapperData;

impl<'s> MapperSystemData<'s> for CharacterSelectionEventStdinMapperData {
    type SystemData = Read<'s, CharacterAssets>;
}

/// Builds a `CharacterSelectionEvent` from stdin tokens.
#[derive(Debug, TypeName)]
pub struct CharacterSelectionEventStdinMapper;

impl CharacterSelectionEventStdinMapper {
    fn map_select_event(
        character_assets: &CharacterAssets,
        controller_id: ControllerId,
        selection: &str,
    ) -> Result<CharacterSelectionEvent, Error> {
        let character_selection = match selection {
            "random" => {
                let snh = SlugAndHandle::from(
                    character_assets
                        .iter()
                        .next()
                        .expect("Expected at least one character to be loaded."),
                );
                CharacterSelection::Random(snh)
            }
            slug_str => {
                let slug = AssetSlug::from_str(slug_str).map_err(StdioError::Msg)?;
                let handle = character_assets
                    .get(&slug)
                    .ok_or_else(|| format!("No character found with asset slug `{}`.", slug))
                    .map_err(StdioError::Msg)?
                    .clone();

                let snh = SlugAndHandle { slug, handle };
                CharacterSelection::Id(snh)
            }
        };

        let character_selection_event = CharacterSelectionEvent::Select {
            controller_id,
            character_selection,
        };

        Ok(character_selection_event)
    }
}

impl StdinMapper for CharacterSelectionEventStdinMapper {
    type SystemData = CharacterSelectionEventStdinMapperData;
    type Event = CharacterSelectionEvent;
    type Args = CharacterSelectionEventArgs;

    fn map(
        character_assets: &Read<CharacterAssets>,
        args: Self::Args,
    ) -> Result<Self::Event, Error> {
        match args {
            CharacterSelectionEventArgs::Select {
                controller_id,
                selection,
            } => Self::map_select_event(character_assets, controller_id, &selection),
            CharacterSelectionEventArgs::Deselect { controller_id } => {
                Ok(CharacterSelectionEvent::Deselect { controller_id })
            }
            CharacterSelectionEventArgs::Confirm => Ok(CharacterSelectionEvent::Confirm),
        }
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Read, Resources},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::loaded::{CharacterAssets, SlugAndHandle};
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
    use stdio_spi::{StdinMapper, StdioError};

    use super::CharacterSelectionEventStdinMapper;
    use crate::CharacterSelectionEventArgs;

    #[test]
    fn returns_err_when_asset_slug_invalid() {
        let controller_id = 0;
        let selection = "invalid".to_string();
        let args = CharacterSelectionEventArgs::Select {
            controller_id,
            selection,
        };
        let mut resources = Resources::new();
        resources.insert(CharacterAssets::new());

        let result = CharacterSelectionEventStdinMapper::map(
            &Read::from(resources.fetch::<CharacterAssets>()),
            args,
        );

        expect_err_msg(
            result,
            "Expected exactly one `/` in slug string: \"invalid\".",
        );
    }

    #[test]
    fn returns_err_when_character_does_not_exist_for_slug() {
        let controller_id = 0;
        let selection = "test/non_existent".to_string();
        let args = CharacterSelectionEventArgs::Select {
            controller_id,
            selection,
        };
        let mut resources = Resources::new();
        resources.insert(CharacterAssets::new());

        let result = CharacterSelectionEventStdinMapper::map(
            &Read::from(resources.fetch::<CharacterAssets>()),
            args,
        );

        expect_err_msg(
            result,
            "No character found with asset slug `test/non_existent`.",
        );
    }

    #[test]
    fn maps_select_id_event() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base("maps_select_id_event", false)
                .with_assertion(|world| {
                    let controller_id = 1;
                    let args = CharacterSelectionEventArgs::Select {
                        controller_id,
                        selection: ASSETS_CHAR_BAT_SLUG.to_string(),
                    };
                    let character_assets = world.read_resource::<CharacterAssets>();
                    let snh =
                        SlugAndHandle::from((&*character_assets, ASSETS_CHAR_BAT_SLUG.clone()));

                    let result = CharacterSelectionEventStdinMapper::map(
                        &Read::from(character_assets),
                        args,
                    );

                    assert!(result.is_ok());
                    let character_selection = CharacterSelection::Id(snh);
                    assert_eq!(
                        CharacterSelectionEvent::Select {
                            controller_id,
                            character_selection
                        },
                        result.unwrap()
                    )
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
                    let controller_id = 1;
                    let args = CharacterSelectionEventArgs::Select {
                        controller_id,
                        selection: "random".to_string(),
                    };
                    let character_assets = world.read_resource::<CharacterAssets>();
                    let snh = SlugAndHandle::from(
                        character_assets
                            .iter()
                            .next()
                            .expect("Expected at least one character to be loaded."),
                    );

                    let result = CharacterSelectionEventStdinMapper::map(
                        &Read::from(character_assets),
                        args,
                    );

                    assert!(result.is_ok());
                    let character_selection = CharacterSelection::Random(snh);
                    assert_eq!(
                        CharacterSelectionEvent::Select {
                            controller_id,
                            character_selection
                        },
                        result.unwrap()
                    )
                })
                .run()
                .is_ok()
        );
    }

    #[test]
    fn maps_deselect_event() {
        let controller_id = 0;
        let args = CharacterSelectionEventArgs::Deselect { controller_id };
        let mut resources = Resources::new();
        resources.insert(CharacterAssets::new());

        let result = CharacterSelectionEventStdinMapper::map(
            &Read::from(resources.fetch::<CharacterAssets>()),
            args,
        );

        assert!(result.is_ok());
        assert_eq!(
            CharacterSelectionEvent::Deselect { controller_id },
            result.unwrap()
        )
    }

    #[test]
    fn maps_confirm_event() {
        let args = CharacterSelectionEventArgs::Confirm;
        let mut resources = Resources::new();
        resources.insert(CharacterAssets::new());

        let result = CharacterSelectionEventStdinMapper::map(
            &Read::from(resources.fetch::<CharacterAssets>()),
            args,
        );

        assert!(result.is_ok());
        assert_eq!(CharacterSelectionEvent::Confirm, result.unwrap())
    }

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
