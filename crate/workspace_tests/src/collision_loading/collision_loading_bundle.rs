#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use collision_model::{
        config::{Body, Interactions},
        loaded::{BodySequence, InteractionsSequence},
    };

    use collision_loading::CollisionLoadingBundle;

    #[test]
    fn bundle_build_adds_body_and_interactions_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(CollisionLoadingBundle::new())
            .with_assertion(|world| {
                // Next line will panic if the Processors aren't added
                world.read_resource::<AssetStorage<Body>>();
                world.read_resource::<AssetStorage<BodySequence>>();
                world.read_resource::<AssetStorage<Interactions>>();
                world.read_resource::<AssetStorage<InteractionsSequence>>();
            })
            .run()
    }
}
