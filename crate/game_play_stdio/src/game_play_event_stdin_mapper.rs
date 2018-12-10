use game_play_model::GamePlayEvent;
use stdio_spi::{Result, StdinMapper};
use typename_derive::TypeName;

use crate::GamePlayEventArgs;

/// Builds a `GamePlayEvent` from stdin tokens.
#[derive(Debug, TypeName)]
pub struct GamePlayEventStdinMapper;

impl StdinMapper for GamePlayEventStdinMapper {
    type Resource = ();
    type Event = GamePlayEvent;
    type Args = GamePlayEventArgs;

    fn map(_: &(), args: Self::Args) -> Result<Self::Event> {
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

#[cfg(test)]
mod tests {
    use game_play_model::GamePlayEvent;
    use stdio_spi::StdinMapper;

    use super::GamePlayEventStdinMapper;
    use crate::GamePlayEventArgs;

    macro_rules! test_mapping {
        ($test_name:ident, $variant:ident) => {
            #[test]
            fn $test_name() {
                let args = GamePlayEventArgs::$variant;

                let result = GamePlayEventStdinMapper::map(&(), args);

                assert!(result.is_ok());
                assert_eq!(GamePlayEvent::$variant, result.unwrap())
            }
        };
    }

    test_mapping!(maps_return_event, Return);
    test_mapping!(maps_restart_event, Restart);
    test_mapping!(maps_pause_event, Pause);
    test_mapping!(maps_resume_event, Resume);
    test_mapping!(maps_end_event, End);
    test_mapping!(maps_end_stats_event, EndStats);
}
