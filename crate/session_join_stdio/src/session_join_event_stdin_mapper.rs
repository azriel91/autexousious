use amethyst::Error;
use session_join_model::SessionJoinEvent;
use stdio_spi::StdinMapper;

/// Builds a `SessionJoinEvent` from stdin tokens.
#[derive(Debug)]
pub struct SessionJoinEventStdinMapper;

impl StdinMapper for SessionJoinEventStdinMapper {
    type SystemData = ();
    type Event = SessionJoinEvent;
    type Args = SessionJoinEvent;

    fn map(_: &(), args: Self::Args) -> Result<Self::Event, Error> {
        Ok(args)
    }
}
