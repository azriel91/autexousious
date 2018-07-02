use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};

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

    use amethyst::{core::transform::TransformBundle, input::InputBundle, ui::UiBundle};
    use amethyst_test_support::prelude::*;
    use game_input::{PlayerActionControl, PlayerAxisControl};

    use super::CharacterSelectionBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        assert!(
            AmethystApplication::blank()
                .with_bundle(TransformBundle::new())
                .with_bundle(InputBundle::<PlayerAxisControl, PlayerActionControl>::new())
                .with_bundle(UiBundle::<PlayerAxisControl, PlayerActionControl>::new())
                .with_bundle(CharacterSelectionBundle)
                .run()
                .is_ok()
        );
    }
}
