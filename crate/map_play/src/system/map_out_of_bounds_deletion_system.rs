use amethyst::{
    ecs::{Entities, Join, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use map_model::play::OutOfBoundsDeleteClock;
use typename_derive::TypeName;

/// Ticks each `HitRepeatTracker`'s clock.
#[derive(Debug, Default, TypeName, new)]
pub struct MapOutOfBoundsDeletionSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapOutOfBoundsDeletionSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `OutOfBoundsDeleteClock` components.
    #[derivative(Debug = "ignore")]
    pub out_of_bounds_delete_clocks: WriteStorage<'s, OutOfBoundsDeleteClock>,
}

impl<'s> System<'s> for MapOutOfBoundsDeletionSystem {
    type SystemData = MapOutOfBoundsDeletionSystemData<'s>;

    fn run(
        &mut self,
        MapOutOfBoundsDeletionSystemData {
            entities,
            mut out_of_bounds_delete_clocks,
        }: Self::SystemData,
    ) {
        (&entities, &mut out_of_bounds_delete_clocks)
            .join()
            .for_each(|(entity, out_of_bounds_delete_clock)| {
                out_of_bounds_delete_clock.tick();
                if out_of_bounds_delete_clock.is_complete() {
                    entities.delete(entity).expect("Failed to delete entity.");
                }
            });
    } // kcov-ignore
}
