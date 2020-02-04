#[cfg(test)]
mod test {
    use amethyst::Error;
    use amethyst_test::AmethystApplication;
    use game_input_model::config::ControlBindings;

    use game_play::GamePlayBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_bundle(GamePlayBundle::new())
            .run()
    }
}
