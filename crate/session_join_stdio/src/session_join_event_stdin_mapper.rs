use amethyst::Error;
use session_join_model::SessionJoinEvent;
use stdio_spi::StdinMapper;

/// Builds a `SessionJoinEvent` from stdin tokens.
#[derive(Debug)]
pub struct SessionJoinEventStdinMapper;

impl StdinMapper for SessionJoinEventStdinMapper {
    type Args = SessionJoinEvent;
    type Event = SessionJoinEvent;
    type SystemData = ();

    fn map(_: &(), args: Self::Args) -> Result<Self::Event, Error> {
        Ok(args)
    }
}
