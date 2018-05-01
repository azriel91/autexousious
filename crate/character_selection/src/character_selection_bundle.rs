use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::*;

use CharacterSelectionSystem;

/// Adds the `CharacterSelectionSystem` to the `World` with id `"character_selection_system"`.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct CharacterSelectionBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterSelectionBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            CharacterSelectionSystem::new(),
            "character_selection_system",
            &["input_system"],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::core::transform::TransformBundle;
    use amethyst::input::InputBundle;
    use amethyst::prelude::*;
    use amethyst::ui::UiBundle;
    use amethyst::Result;
    use game_input::{PlayerActionControl, PlayerAxisControl};

    use super::CharacterSelectionBundle;

    fn setup<'a, 'b>() -> Result<Application<'a, 'b>> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));
        let app = Application::build(format!("{}/assets", env!("CARGO_MANIFEST_DIR")), MockState)?
            .with_bundle(TransformBundle::new())?
            .with_bundle(InputBundle::<PlayerAxisControl, PlayerActionControl>::new())?
            .with_bundle(UiBundle::<PlayerAxisControl, PlayerActionControl>::new())?
            .with_bundle(CharacterSelectionBundle)?
            .build()?;

        Ok(app)
    } // kcov-ignore

    #[test]
    fn bundle_build_should_succeed() {
        setup().expect("CharacterSelectionBundle#build() should succeed");
    }

    #[derive(Debug)]
    struct MockState;
    impl State for MockState {}
}
