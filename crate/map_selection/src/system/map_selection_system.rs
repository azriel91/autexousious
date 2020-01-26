use amethyst::{
    ecs::{Read, System, World, Write, WriteExpect},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{config::AssetType, loaded::AssetTypeMappings, play::AssetSelectionEvent};
use derivative::Derivative;
use derive_new::new;
use log::warn;
use map_selection_model::MapSelection;

use crate::MapSelectionStatus;

/// Updates the `MapSelection` resource based on user selection.
#[derive(Debug, Default, new)]
pub struct MapSelectionSystem {
    /// ID for reading map selection events.
    #[new(default)]
    asset_selection_event_rid: Option<ReaderId<AssetSelectionEvent>>,
}

/// `MapSelectionSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSelectionSystemData<'s> {
    /// `MapSelectionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection_status: Write<'s, MapSelectionStatus>,
    /// `AssetSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub asset_selection_ec: Read<'s, EventChannel<AssetSelectionEvent>>,
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
            asset_selection_ec,
            mut map_selection,
        }: Self::SystemData,
    ) {
        asset_selection_ec
            .read(
                self.asset_selection_event_rid
                    .as_mut()
                    .expect("Expected `asset_selection_event_rid` to be set."),
            )
            .copied()
            .for_each(|ev| match ev {
                AssetSelectionEvent::Return => {}
                AssetSelectionEvent::Switch {
                    asset_selection, ..
                }
                | AssetSelectionEvent::Select {
                    asset_selection, ..
                } => {
                    *map_selection = MapSelection::from(asset_selection);
                }
                AssetSelectionEvent::Deselect { .. } => {
                    *map_selection_status = MapSelectionStatus::Pending;
                }
                AssetSelectionEvent::Confirm => {
                    *map_selection_status = MapSelectionStatus::Confirmed;
                }
                AssetSelectionEvent::Join { .. } | AssetSelectionEvent::Leave { .. } => {
                    warn!("Received `{:?}` in `MapSelectionSystem`.", ev);
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

        let mut asset_selection_ec = world.fetch_mut::<EventChannel<AssetSelectionEvent>>();
        self.asset_selection_event_rid = Some(asset_selection_ec.register_reader());
    }
}
