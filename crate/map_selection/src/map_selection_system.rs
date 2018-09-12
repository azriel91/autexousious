use amethyst::{ecs::prelude::*, shrev::EventChannel};

use game_model::loaded::{MapAssets, SlugAndHandle};

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
    WriteExpect<'s, MapSelection>,
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

        if let Some(MapSelectionEvent::Select {
            map_selection: selection,
        }) = events.next()
        {
            *map_selection_status = MapSelectionStatus::Confirmed;
            *map_selection = selection.clone();

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

        if res.try_fetch::<MapSelection>().is_none() {
            let slug_and_handle = res
                .fetch::<MapAssets>()
                .iter()
                .next()
                .map(SlugAndHandle::from)
                .expect("Expected at least one map to be loaded.");

            res.insert(MapSelection::Random(slug_and_handle));
        }

        let mut selection_event_channel = res.fetch_mut::<EventChannel<MapSelectionEvent>>();
        self.reader_id = Some(selection_event_channel.register_reader());
    }
}
