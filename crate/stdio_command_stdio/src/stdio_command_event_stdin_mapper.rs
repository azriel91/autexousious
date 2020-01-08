use amethyst::Error;
use stdio_command_model::{StdioCommandEvent, StdioCommandEventArgs};
use stdio_spi::StdinMapper;

/// Builds a `StdioCommandEvent` from stdin tokens.
#[derive(Debug)]
pub struct StdioCommandEventStdinMapper;

impl StdinMapper for StdioCommandEventStdinMapper {
    type SystemData = ();
    type Event = StdioCommandEvent;
    type Args = StdioCommandEventArgs;

    fn map(_: &(), args: Self::Args) -> Result<Self::Event, Error> {
        Ok(args)
    }
}
