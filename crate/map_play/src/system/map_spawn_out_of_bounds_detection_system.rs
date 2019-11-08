use amethyst::{
    ecs::{Entity, Read, ReadStorage, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use enumflags2::BitFlags;
use kinematic_model::config::Position;
use map_model::{
    loaded::{AssetMargins, Margins},
    play::{BoundaryFace, MapBoundaryEvent, MapBoundaryEventData},
};
use map_selection_model::MapSelection;
use spawn_model::play::SpawnEvent;
use typename_derive::TypeName;

use crate::MapBoundsChecks;

/// Sends a `MapBoundaryEvent` when an entity's `Position` has entered or exited map bounds.
#[derive(Debug, Default, TypeName, new)]
pub struct MapSpawnOutOfBoundsDetectionSystem {
    /// Reader ID for the `SpawnEvent` channel.
    #[new(default)]
    spawn_event_rid: Option<ReaderId<SpawnEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSpawnOutOfBoundsDetectionSystemData<'s> {
    /// `SpawnEvent` channel.
    #[derivative(Debug = "ignore")]
    pub spawn_ec: Read<'s, EventChannel<SpawnEvent>>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: Read<'s, MapSelection>,
    /// `AssetMargins` resource.
    #[derivative(Debug = "ignore")]
    pub asset_margins: Read<'s, AssetMargins>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `MapBoundaryEvent` channel.
    #[derivative(Debug = "ignore")]
    pub map_boundary_ec: Write<'s, EventChannel<MapBoundaryEvent>>,
}

impl MapSpawnOutOfBoundsDetectionSystem {
    /// Returns a `MapBoundaryEvent` if the entity has crossed the map boundary.
    fn detect_enter_exit(
        map_margins: &Margins,
        entity: Entity,
        position: Position<f32>,
    ) -> Option<MapBoundaryEvent> {
        let (within_x, within_y, within_z) =
            MapBoundsChecks::position_comparative(map_margins, position);
        let within_bounds = MapBoundsChecks::is_within_bounds(within_x, within_y, within_z);

        if !within_bounds {
            Some(MapBoundaryEvent::Exit(MapBoundaryEventData {
                entity,
                boundary_faces: BitFlags::<BoundaryFace>::default(),
            }))
        } else {
            None
        }
    }
}

impl<'s> System<'s> for MapSpawnOutOfBoundsDetectionSystem {
    type SystemData = MapSpawnOutOfBoundsDetectionSystemData<'s>;

    fn run(
        &mut self,
        MapSpawnOutOfBoundsDetectionSystemData {
            spawn_ec,
            map_selection,
            asset_margins,
            positions,
            mut map_boundary_ec,
        }: Self::SystemData,
    ) {
        let map_margins = asset_margins
            .get(
                map_selection
                    .asset_id()
                    .expect("Expected `MapSelection` asset ID to exist."),
            )
            .expect("Expected `Margins` to be loaded.");

        // Send event when the entity is spawned out of bounds.
        let spawn_event_rid = self
            .spawn_event_rid
            .as_mut()
            .expect("Expected `spawn_event_rid` field to be set.");

        let map_boundary_events = spawn_ec
            .read(spawn_event_rid)
            .filter_map(|ev| {
                let entity = ev.entity_spawned;
                let position = positions.get(entity);

                position
                    .and_then(|position| Self::detect_enter_exit(map_margins, entity, *position))
            })
            .collect::<Vec<MapBoundaryEvent>>();
        map_boundary_ec.iter_write(map_boundary_events);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.spawn_event_rid = Some(
            world
                .fetch_mut::<EventChannel<SpawnEvent>>()
                .register_reader(),
        );
    }
}
