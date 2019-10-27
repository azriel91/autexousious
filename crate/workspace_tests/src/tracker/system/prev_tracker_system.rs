#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{SystemData, World, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;

    use tracker::{Prev, PrevTrackerSystem, PrevTrackerSystemData};

    #[test]
    fn inserts_prev_resource_with_cloned_value() -> Result<(), Error> {
        let system = PrevTrackerSystem::<TestResource>::new(stringify!(TestResource));
        let system_name = system.system_name();

        AmethystApplication::blank()
            .with_effect(PrevTrackerSystemData::<TestResource>::setup)
            .with_effect(|world| world.insert(TestResource(123)))
            .with_system_single(system.clone(), &system_name, &[])
            .with_assertion(|world| assert_prev_value(world, 123))
            .with_effect(|world| {
                let mut test_resource = world.write_resource::<TestResource>();

                test_resource.0 = 456;
            })
            .with_system_single(system, &system_name, &[])
            .with_assertion(|world| assert_prev_value(world, 456))
            .run()
    }

    fn assert_prev_value(world: &mut World, value: i32) {
        let test_resource_prev = world.read_resource::<Prev<TestResource>>();

        assert_eq!(value, (test_resource_prev.0).0);
    }

    #[derive(Clone)]
    struct TestResource(pub i32);
}
