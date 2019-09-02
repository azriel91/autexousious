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

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{world::EntitiesRes, Builder, Entity, System, SystemData, World, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use map_model::play::OutOfBoundsDeleteClock;

    use super::MapOutOfBoundsDeletionSystem;

    #[test]
    fn ticks_out_of_bounds_delete_clock() -> Result<(), Error> {
        run_test(
            SetupParams {
                out_of_bounds_delete_clock: OutOfBoundsDeleteClock::new(10),
            },
            ExpectedParams {
                entity_state: EntityState::Alive(OutOfBoundsDeleteClock::new_with_value(10, 1)),
            },
        )
    }

    #[test]
    fn deletes_entities_with_completed_clocks() -> Result<(), Error> {
        run_test(
            SetupParams {
                out_of_bounds_delete_clock: OutOfBoundsDeleteClock::new_with_value(10, 10),
            },
            ExpectedParams {
                entity_state: EntityState::Dead,
            },
        )
    }

    fn run_test(
        SetupParams {
            out_of_bounds_delete_clock,
        }: SetupParams,
        ExpectedParams { entity_state }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(setup_system_data)
            .with_effect(move |world| {
                let entity = world
                    .create_entity()
                    .with(out_of_bounds_delete_clock)
                    .build();

                world.insert(entity);
            })
            .with_system_single(MapOutOfBoundsDeletionSystem::new(), "", &[])
            .with_effect(|world| world.maintain())
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let entities = world.read_resource::<EntitiesRes>();
                match entity_state {
                    EntityState::Alive(out_of_bounds_delete_clock_expected) => {
                        let out_of_bounds_delete_clocks =
                            world.read_storage::<OutOfBoundsDeleteClock>();
                        let out_of_bounds_delete_clock_actual = out_of_bounds_delete_clocks
                            .get(entity)
                            .copied()
                            .expect("Expected entity to have `OutOfBoundsDeleteClock` component.");

                        assert!(entities.is_alive(entity));
                        assert_eq!(
                            out_of_bounds_delete_clock_expected,
                            out_of_bounds_delete_clock_actual
                        );
                    }
                    EntityState::Dead => {
                        assert!(!entities.is_alive(entity));
                    }
                }
            })
            .run()
    }

    fn setup_system_data(world: &mut World) {
        <MapOutOfBoundsDeletionSystem as System<'_>>::SystemData::setup(world);
    }

    struct SetupParams {
        out_of_bounds_delete_clock: OutOfBoundsDeleteClock,
    }

    struct ExpectedParams {
        entity_state: EntityState,
    }

    enum EntityState {
        Alive(OutOfBoundsDeleteClock),
        Dead,
    }
}
