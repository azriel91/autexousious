#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use ui_model::config::UiDefinition;

    use ui_loading::UiLoadingBundle;

    #[test]
    fn bundle_build_adds_ui_definition_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(UiLoadingBundle::new())
            .with_assertion(|world| {
                world.read_resource::<AssetStorage<UiDefinition>>();
            })
            .run()
    }
}
