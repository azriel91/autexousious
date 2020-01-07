use amethyst::{
    ecs::{Read, System, World, Write, WriteExpect},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{config::AssetType, loaded::AssetTypeMappings};
use derivative::Derivative;
use derive_new::new;
use map_selection_model::{MapSelection, MapSelectionEvent};

use crate::MapSelectionStatus;

/// Updates the `MapSelection` resource based on user selection.
#[derive(Debug, Default, new)]
pub struct MapSelectionSystem {
    /// ID for reading map selection events.
    #[new(default)]
    map_selection_event_rid: Option<ReaderId<MapSelectionEvent>>,
}

/// `MapSelectionSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSelectionSystemData<'s> {
    /// `MapSelectionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection_status: Write<'s, MapSelectionStatus>,
    /// `MapSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub map_selection_ec: Read<'s, EventChannel<MapSelectionEvent>>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: WriteExpect<'s, MapSelection>,
}

impl<'s> System<'s> for MapSelectionSystem {
    type SystemData = MapSelectionSystemData<'s>;

    fn run(
        &mut self,
        MapSelectionSystemData {
            mut map_selection_status,
            map_selection_ec,
            mut map_selection,
        }: Self::SystemData,
    ) {
        map_selection_ec
            .read(
                self.map_selection_event_rid
                    .as_mut()
                    .expect("Expected `map_selection_event_rid` to be set."),
            )
            .for_each(|ev| match ev {
                MapSelectionEvent::Return => {}
                MapSelectionEvent::Switch {
                    map_selection: selection,
                }
                | MapSelectionEvent::Select {
                    map_selection: selection,
                } => {
                    *map_selection = *selection;
                }
                MapSelectionEvent::Deselect => {
                    *map_selection_status = MapSelectionStatus::Pending;
                }
                MapSelectionEvent::Confirm => {
                    *map_selection_status = MapSelectionStatus::Confirmed;
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        if world.try_fetch::<MapSelection>().is_none() {
            let first_map_id = world
                .fetch::<AssetTypeMappings>()
                .iter_ids(&AssetType::Map)
                .next()
                .copied()
                .expect("Expected at least one map to be loaded.");

            world.insert(MapSelection::Random(Some(first_map_id)));
        }

        let mut map_selection_ec = world.fetch_mut::<EventChannel<MapSelectionEvent>>();
        self.map_selection_event_rid = Some(map_selection_ec.register_reader());
    }
}
