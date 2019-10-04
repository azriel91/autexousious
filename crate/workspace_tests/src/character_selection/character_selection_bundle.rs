#[cfg(test)]
mod test {
    use amethyst::{core::transform::TransformBundle, Error};
    use amethyst_test::AmethystApplication;

    use character_selection::CharacterSelectionBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(
                CharacterSelectionBundle::new()
                    .with_system_dependencies(&["transform_system".to_string()]),
            )
            .run()
    }
}
