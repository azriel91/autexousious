#[cfg(test)]
mod test {
    use amethyst::{input::StringBindings, Error};
    use amethyst_test::AmethystApplication;

    use game_mode_selection_ui::GameModeSelectionUiBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        AmethystApplication::ui_base::<StringBindings>()
            .with_bundle(GameModeSelectionUiBundle::new())
            .run()
    }
}
