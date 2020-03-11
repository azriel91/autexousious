use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Errors when using `game_input_model` types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameInputModelError {
    /// Failed to parse a `PlayerController` from string.
    PlayerControllerParseError,
    /// Failed to parse `PlayerControllers` from string.
    PlayerControllersParseError,
}

impl Display for GameInputModelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlayerControllerParseError => write!(
                f,
                "Player controller must be in the form `<id>:<name>`. Example: `0:azriel`"
            ),
            Self::PlayerControllersParseError => write!(
                f,
                "Player controllers must be space separated in the form \
                    `<id>:<name> <id>:<name>`. Example: `0:azriel 1:friend`"
            ),
        }
    }
}

impl Error for GameInputModelError {}
