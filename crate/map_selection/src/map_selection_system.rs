use amethyst::{ecs::prelude::*, shrev::EventChannel};

use MapSelection;
use MapSelectionEvent;
use MapSelectionStatus;

/// Updates the `MapSelection` resource based on user selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionSystem {
    /// ID for reading map selection events.
    #[new(default)]
    reader_id: Option<ReaderId<MapSelectionEvent>>,
}

type MapSelectionSystemData<'s, 'c> = (
    Write<'s, MapSelectionStatus>,
    Read<'s, EventChannel<MapSelectionEvent>>,
    Write<'s, MapSelection>,
);

impl<'s> System<'s> for MapSelectionSystem {
    type SystemData = MapSelectionSystemData<'s, 's>;

    fn run(
        &mut self,
        (mut map_selection_status, selection_event_channel, mut map_selection): Self::SystemData,
    ) {
        if let MapSelectionStatus::Confirmed = *map_selection_status {
            return;
        }

        let mut events = selection_event_channel.read(self.reader_id.as_mut().unwrap());

        if let Some(MapSelectionEvent { map_handle }) = events.next() {
            *map_selection_status = MapSelectionStatus::Confirmed;
            map_selection.map_handle = Some(map_handle.clone());

            // Discard additional events, and log a message
            let additional_events = events.count();
            if additional_events > 0 {
                warn!(
                    "Discarding `{}` additional map selection events.",
                    additional_events
                );
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        let mut selection_event_channel = res.fetch_mut::<EventChannel<MapSelectionEvent>>();
        self.reader_id = Some(selection_event_channel.register_reader());
    }
}
