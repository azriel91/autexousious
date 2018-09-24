use std::str::FromStr;

use application_event::{AppEvent, AppEventVariant};
use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
use game_model::{
    config::AssetSlugBuilder,
    loaded::{CharacterAssets, MapAssets, SlugAndHandle},
};
use map_selection_model::{MapSelection, MapSelectionEvent};

use itertools::Itertools;
use shell_words::{self, ParseError};
use strum::IntoEnumIterator;

/// Functions to parse input into an `AppEvent`.
#[derive(Debug)]
pub struct IoAppEventUtils;

/// Resources used to construct `AppEvent`s.
pub type Resources<'res> = (&'res CharacterAssets, &'res MapAssets);

impl IoAppEventUtils {
    pub fn handle_input(resources: Resources, input: String) -> Result<Option<AppEvent>, String> {
        shell_words::split(&input)
            .map_err(|e| Self::parse_error_to_string(&input, e))
            .and_then(Self::tokens_to_variant)
            .and_then(|variant_and_tokens| Self::variant_to_event(resources, variant_and_tokens))
    }

    fn parse_error_to_string(input: &str, e: ParseError) -> String {
        format!(
            "Error splitting input string. Error:\n\
             \n\
             ```\n\
             {}\n\
             ```\n\
             \n\
             Input:\n\
             ```\n\
             {:?}\n\
             ```\n\n",
            e, input
        )
    }

    fn tokens_to_variant(
        tokens: Vec<String>,
    ) -> Result<Option<(AppEventVariant, Vec<String>)>, String> {
        if tokens.is_empty() {
            Ok(None)
        } else {
            Self::cmd_to_variant(&tokens[0]).map(|variant| Some((variant, tokens)))
        }
    }

    fn cmd_to_variant(cmd: &str) -> Result<AppEventVariant, String> {
        AppEventVariant::from_str(cmd).map_err(|e| {
            format!(
                "Error parsing `{}` as an {}. Error: `{}`.\n\
                 Valid values are: {:?}",
                cmd,
                stringify!(AppEventVariant),
                e,
                AppEventVariant::iter().join(", ")
            )
        })
    }

    fn variant_to_event(
        (character_assets, map_assets): Resources,
        variant_and_tokens: Option<(AppEventVariant, Vec<String>)>,
    ) -> Result<Option<AppEvent>, String> {
        let event = variant_and_tokens.map(|(variant, mut tokens)| match variant {
            // TODO: Robust handling.
            AppEventVariant::CharacterSelection => {
                let mut args = tokens.drain(1..);
                let slug = AssetSlugBuilder::default()
                    .namespace(args.next().unwrap())
                    .name(args.next().unwrap())
                    .build()
                    .unwrap();
                let handle = character_assets.get(&slug).unwrap().clone();
                let snh = SlugAndHandle { slug, handle };
                let character_selection = CharacterSelection::Id(snh);
                let character_selection_event = CharacterSelectionEvent::Select {
                    controller_id: 0,
                    character_selection,
                };
                AppEvent::CharacterSelection(character_selection_event)
            }
            AppEventVariant::MapSelection => {
                let mut args = tokens.drain(1..);
                let slug = AssetSlugBuilder::default()
                    .namespace(args.next().unwrap())
                    .name(args.next().unwrap())
                    .build()
                    .unwrap();
                let handle = map_assets.get(&slug).unwrap().clone();
                let snh = SlugAndHandle { slug, handle };
                let map_selection = MapSelection::Id(snh);
                let map_selection_event = MapSelectionEvent::Select { map_selection };
                AppEvent::MapSelection(map_selection_event)
            }
        });

        Ok(event)
    }
}
