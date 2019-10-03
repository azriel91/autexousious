use amethyst::Error;
use application_menu::MenuEvent;
use game_mode_selection_model::GameModeSelectionEvent;
use stdio_spi::StdinMapper;
use typename_derive::TypeName;

use crate::GameModeSelectionEventArgs;

/// Builds a `GameModeSelectionEvent` from stdin tokens.
#[derive(Debug, TypeName)]
pub struct GameModeSelectionEventStdinMapper;

impl StdinMapper for GameModeSelectionEventStdinMapper {
    type SystemData = ();
    type Event = GameModeSelectionEvent;
    type Args = GameModeSelectionEventArgs;

    fn map(_: &(), args: Self::Args) -> Result<Self::Event, Error> {
        match args {
            GameModeSelectionEventArgs::Select { index } => Ok(MenuEvent::Select(index)),
            GameModeSelectionEventArgs::Close => Ok(MenuEvent::Close),
        }
    }
}
