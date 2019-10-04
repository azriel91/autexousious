#[cfg(test)]
mod test {
    use amethyst::Error;
    use amethyst_test::AmethystApplication;
    use game_input::GameInputBundle;
    use game_input_model::ControlBindings;

    use character_selection_ui::CharacterSelectionUiBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_bundle(GameInputBundle::new())
            .with_bundle(CharacterSelectionUiBundle::new())
            .run()
    }
}
