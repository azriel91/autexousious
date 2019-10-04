use amethyst::Error;
use game_play_model::GamePlayEvent;
use stdio_spi::StdinMapper;
use typename_derive::TypeName;

use crate::GamePlayEventArgs;

/// Builds a `GamePlayEvent` from stdin tokens.
#[derive(Debug, TypeName)]
pub struct GamePlayEventStdinMapper;

impl StdinMapper for GamePlayEventStdinMapper {
    type SystemData = ();
    type Event = GamePlayEvent;
    type Args = GamePlayEventArgs;

    fn map(_: &(), args: Self::Args) -> Result<Self::Event, Error> {
        match args {
            GamePlayEventArgs::Return => Ok(GamePlayEvent::Return),
            GamePlayEventArgs::Restart => Ok(GamePlayEvent::Restart),
            GamePlayEventArgs::Pause => Ok(GamePlayEvent::Pause),
            GamePlayEventArgs::Resume => Ok(GamePlayEvent::Resume),
            GamePlayEventArgs::End => Ok(GamePlayEvent::End),
            GamePlayEventArgs::EndStats => Ok(GamePlayEvent::EndStats),
        }
    }
}
