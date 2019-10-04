#[cfg(test)]
mod test {
    use amethyst::Error;
    use amethyst_test::AmethystApplication;
    use game_input::GameInputBundle;
    use game_input_model::ControlBindings;

    use map_selection_ui::MapSelectionUiBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_bundle(GameInputBundle::new())
            .with_bundle(MapSelectionUiBundle::new())
            .run()
    }
}
