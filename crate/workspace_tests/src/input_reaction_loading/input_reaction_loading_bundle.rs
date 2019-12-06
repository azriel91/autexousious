#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use input_reaction_model::loaded::{InputReactions, InputReactionsSequence};

    use input_reaction_loading::InputReactionLoadingBundle;

    #[test]
    fn bundle_build() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(InputReactionLoadingBundle::new())
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<InputReactions>>();
                world.read_resource::<AssetStorage<InputReactionsSequence>>();
            })
            .run()
    }
}
