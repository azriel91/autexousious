use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use map_model::play::{
    MapBoundaryEvent, MapBoundaryEventData, MapUnboundedDelete, OutOfBoundsDeleteClock,
};

/// Number of ticks an entity has to remain out of bounds before it is deleted.
pub const OUT_OF_BOUNDS_DELETE_DELAY: usize = 180;

/// Adds/removes `OutOfBoundsDeleteClock`s to `MapUnboundedDelete` entities.
#[derive(Debug, Default, new)]
pub struct MapOutOfBoundsClockAugmentSystem {
    /// Reader ID for the `MapBoundaryEvent` channel.
    #[new(default)]
    map_boundary_event_rid: Option<ReaderId<MapBoundaryEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapOutOfBoundsClockAugmentSystemData<'s> {
    /// `MapBoundaryEvent` channel.
    #[derivative(Debug = "ignore")]
    pub map_boundary_ec: Read<'s, EventChannel<MapBoundaryEvent>>,
    /// `MapUnboundedDelete` components.
    #[derivative(Debug = "ignore")]
    pub map_unbounded_deletes: ReadStorage<'s, MapUnboundedDelete>,
    /// `OutOfBoundsDeleteClock` components.
    #[derivative(Debug = "ignore")]
    pub out_of_bounds_delete_clocks: WriteStorage<'s, OutOfBoundsDeleteClock>,
}

impl<'s> System<'s> for MapOutOfBoundsClockAugmentSystem {
    type SystemData = MapOutOfBoundsClockAugmentSystemData<'s>;

    fn run(
        &mut self,
        MapOutOfBoundsClockAugmentSystemData {
            map_boundary_ec,
            map_unbounded_deletes,
            mut out_of_bounds_delete_clocks,
        }: Self::SystemData,
    ) {
        let map_boundary_event_rid = self
            .map_boundary_event_rid
            .as_mut()
            .expect("Expected `map_boundary_event_rid` field to be set.");

        map_boundary_ec
            .read(map_boundary_event_rid)
            .for_each(|ev| match ev {
                MapBoundaryEvent::Exit(MapBoundaryEventData { entity, .. }) => {
                    let entity = *entity;
                    if map_unbounded_deletes.contains(entity) {
                        out_of_bounds_delete_clocks
                            .insert(
                                entity,
                                OutOfBoundsDeleteClock::new(OUT_OF_BOUNDS_DELETE_DELAY),
                            )
                            .expect("Failed to insert `OutOfBoundsDeleteClock` component.");
                    }
                }
                MapBoundaryEvent::Enter(MapBoundaryEventData { entity, .. }) => {
                    let entity = *entity;
                    if map_unbounded_deletes.contains(entity)
                        && out_of_bounds_delete_clocks.contains(entity)
                    {
                        out_of_bounds_delete_clocks.remove(entity);
                    }
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.map_boundary_event_rid = Some(
            world
                .fetch_mut::<EventChannel<MapBoundaryEvent>>()
                .register_reader(),
        );
    }
}
