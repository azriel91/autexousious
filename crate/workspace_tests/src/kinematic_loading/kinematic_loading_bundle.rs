#[cfg(test)]
mod tests {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use kinematic_model::loaded::ObjectAccelerationSequence;

    use kinematic_loading::KinematicLoadingBundle;

    #[test]
    fn bundle_build_adds_object_acceleration_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(KinematicLoadingBundle::new())
            .with_assertion(|world| {
                // Next line will panic if the Processors aren't added
                world.read_resource::<AssetStorage<ObjectAccelerationSequence>>();
            })
            .run()
    }
}
