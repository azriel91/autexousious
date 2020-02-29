use amethyst::Error;
use session_host_model::SessionHostEvent;
use stdio_spi::StdinMapper;

/// Builds a `SessionHostEvent` from stdin tokens.
#[derive(Debug)]
pub struct SessionHostEventStdinMapper;

impl StdinMapper for SessionHostEventStdinMapper {
    type SystemData = ();
    type Event = SessionHostEvent;
    type Args = SessionHostEvent;

    fn map(_: &(), args: Self::Args) -> Result<Self::Event, Error> {
        Ok(args)
    }
}
