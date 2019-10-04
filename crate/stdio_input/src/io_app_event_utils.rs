use std::str::FromStr;

use application_event::AppEventVariant;
use itertools::Itertools;
use shell_words::{self, ParseError};
use stdio_spi::VariantAndTokens;
use strum::IntoEnumIterator;

/// Functions to parse input into an `AppEvent`.
#[derive(Debug)]
pub struct IoAppEventUtils;

impl IoAppEventUtils {
    /// Maps the input string to an `AppEventVariant` and `String` tokens.
    pub fn input_to_variant_and_tokens(input: &str) -> Result<Option<VariantAndTokens>, String> {
        shell_words::split(&input)
            .map_err(|e| Self::parse_error_to_string(&input, e))
            .and_then(Self::tokens_to_variant)
    }

    fn parse_error_to_string(input: &str, e: ParseError) -> String {
        format!(
            "Error splitting input string. Input:\n\
             \n\
             ```\n\
             {}\n\
             ```\n\
             \n\
             Error:\n\
             ```\n\
             {}\n\
             ```\n\n",
            input, e
        )
    }

    fn tokens_to_variant(tokens: Vec<String>) -> Result<Option<VariantAndTokens>, String> {
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
}
