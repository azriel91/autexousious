use amethyst::Error;
use menu_model::MenuEvent;
use network_mode_selection_model::{NetworkModeSelectionEvent, NetworkModeSelectionEventArgs};
use stdio_spi::StdinMapper;

/// Builds a `NetworkModeSelectionEvent` from stdin tokens.
#[derive(Debug)]
pub struct NetworkModeSelectionEventStdinMapper;

impl StdinMapper for NetworkModeSelectionEventStdinMapper {
    type SystemData = ();
    type Event = NetworkModeSelectionEvent;
    type Args = NetworkModeSelectionEventArgs;

    fn map(_: &(), args: Self::Args) -> Result<Self::Event, Error> {
        match args {
            NetworkModeSelectionEventArgs::Select { index } => Ok(MenuEvent::Select(index)),
            NetworkModeSelectionEventArgs::Close => Ok(MenuEvent::Close),
        }
    }
}
