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
