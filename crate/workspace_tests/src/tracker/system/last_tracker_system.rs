#[cfg(test)]
mod test {
    use amethyst::ecs::{
        storage::VecStorage, Builder, Component, Entity, SystemData, World, WorldExt,
    };
    use amethyst_test::AmethystApplication;

    use tracker::{Last, LastTrackerSystem, LastTrackerSystemData};

    #[test]
    fn inserts_last_component_with_cloned_value() {
        let system = LastTrackerSystem::<TestComponent>::new(stringify!(TestComponent));
        let system_name = system.system_name();
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_effect(setup_components)
                .with_effect(|world| {
                    let entity = world.create_entity().with(TestComponent(123)).build();
                    world.insert(entity);
                })
                .with_system_single(system.clone(), &system_name, &[])
                .with_assertion(|world| assert_last_value(world, 123))
                .with_effect(|world| {
                    let entity = *world.read_resource::<Entity>();

                    let test_components = &mut world.write_storage::<TestComponent>();
                    let test_component = test_components
                        .get_mut(entity)
                        .expect("Entity should have a `TestComponent` component.");

                    test_component.0 = 456;
                })
                .with_system_single(system, &system_name, &[])
                .with_assertion(|world| assert_last_value(world, 456))
                .run()
                .is_ok()
        );
    }

    fn setup_components(world: &mut World) {
        LastTrackerSystemData::<TestComponent>::setup(world);
    }

    fn assert_last_value(world: &mut World, value: i32) {
        let entity = *world.read_resource::<Entity>();

        let last_test_components = world.read_storage::<Last<TestComponent>>();
        let last_test_component = last_test_components
            .get(entity)
            .expect("Entity should have a `Last<TestComponent>` component.");

        assert_eq!(value, (last_test_component.0).0);
    }

    #[derive(Clone, Component)]
    #[storage(VecStorage)]
    struct TestComponent(pub i32);
}
