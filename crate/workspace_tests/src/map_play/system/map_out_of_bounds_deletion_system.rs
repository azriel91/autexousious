#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{world::EntitiesRes, Builder, Entity, System, SystemData, World, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use map_model::play::OutOfBoundsDeleteClock;

    use map_play::MapOutOfBoundsDeletionSystem;

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
