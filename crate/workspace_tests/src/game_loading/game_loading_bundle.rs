#[cfg(test)]
mod test {
    use amethyst::Error;
    use amethyst_test::AmethystApplication;

    use game_loading::GameLoadingBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(GameLoadingBundle::new())
            .run()
    }
}
