use application_menu::MenuEvent;
use game_mode_selection_model::GameModeSelectionEvent;
use stdio_spi::{Result, StdinMapper};

use GameModeSelectionEventArgs;

/// Builds a `GameModeSelectionEvent` from stdin tokens.
#[derive(Debug, TypeName)]
pub struct GameModeSelectionEventStdinMapper;

impl StdinMapper for GameModeSelectionEventStdinMapper {
    type Resource = ();
    type Event = GameModeSelectionEvent;
    type Args = GameModeSelectionEventArgs;

    fn map(_: &(), args: Self::Args) -> Result<Self::Event> {
        match args {
            GameModeSelectionEventArgs::Select { index } => Ok(MenuEvent::Select(index)),
            GameModeSelectionEventArgs::Close => Ok(MenuEvent::Close),
        }
    }
}

#[cfg(test)]
mod tests {
    use application_menu::MenuEvent;
    use game_mode_selection_model::GameModeIndex;
    use stdio_spi::StdinMapper;

    use super::GameModeSelectionEventStdinMapper;
    use GameModeSelectionEventArgs;

    #[test]
    fn maps_select_event() {
        let args = GameModeSelectionEventArgs::Select {
            index: GameModeIndex::StartGame,
        };

        let result = GameModeSelectionEventStdinMapper::map(&(), args);

        assert!(result.is_ok());
        assert_eq!(MenuEvent::Select(GameModeIndex::StartGame), result.unwrap())
    }

    #[test]
    fn maps_close_event() {
        let args = GameModeSelectionEventArgs::Close;

        let result = GameModeSelectionEventStdinMapper::map(&(), args);

        assert!(result.is_ok());
        assert_eq!(MenuEvent::Close, result.unwrap())
    }
}
