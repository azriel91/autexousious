use amethyst::Error;
use network_join_model::NetworkJoinEvent;
use stdio_spi::StdinMapper;

/// Builds a `NetworkJoinEvent` from stdin tokens.
#[derive(Debug)]
pub struct NetworkJoinEventStdinMapper;

impl StdinMapper for NetworkJoinEventStdinMapper {
    type SystemData = ();
    type Event = NetworkJoinEvent;
    type Args = NetworkJoinEvent;

    fn map(_: &(), args: Self::Args) -> Result<Self::Event, Error> {
        Ok(args)
    }
}
