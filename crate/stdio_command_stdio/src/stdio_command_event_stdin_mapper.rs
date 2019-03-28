use amethyst::Error;
use stdio_command_model::StdioCommandEvent;
use stdio_spi::StdinMapper;
use typename_derive::TypeName;

use crate::StdioCommandEventArgs;

/// Builds a `StdioCommandEvent` from stdin tokens.
#[derive(Debug, TypeName)]
pub struct StdioCommandEventStdinMapper;

impl StdinMapper for StdioCommandEventStdinMapper {
    type SystemData = ();
    type Event = StdioCommandEvent;
    type Args = StdioCommandEventArgs;

    fn map(_: &(), args: Self::Args) -> Result<Self::Event, Error> {
        Ok(args)
    }
}

#[cfg(test)]
mod tests {
    use state_registry::StateId;
    use stdio_command_model::{StateBarrier, StdioCommandEvent};
    use stdio_spi::StdinMapper;

    use super::StdioCommandEventStdinMapper;

    #[test]
    fn maps_command_barrier_event() {
        let args = StdioCommandEvent::StateBarrier(StateBarrier::new(StateId::GamePlay));

        let result = StdioCommandEventStdinMapper::map(&(), args);

        assert!(result.is_ok());
        assert_eq!(
            StdioCommandEvent::StateBarrier(StateBarrier::new(StateId::GamePlay)),
            result.unwrap()
        )
    }
}
