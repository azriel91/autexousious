#[cfg(test)]
mod test {
    use amethyst::Error;
    use amethyst_test::AmethystApplication;
    use game_input_model::{ControlBindings, InputConfig};

    use game_input_ui::GameInputUiBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_bundle(GameInputUiBundle::new(InputConfig::default()))
            .run()
    }
}
