use amethyst::Error;
use game_mode_selection_model::{GameModeSelectionEvent, GameModeSelectionEventArgs};
use menu_model::MenuEvent;
use stdio_spi::StdinMapper;

/// Builds a `GameModeSelectionEvent` from stdin tokens.
#[derive(Debug)]
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
