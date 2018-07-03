use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};

use CharacterInputUpdateSystem;

/// Adds the `CharacterInputUpdateSystem` to the `World` with id `"character_input_update_system"`.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct GamePlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            CharacterInputUpdateSystem::new(),
            "character_input_update_system",
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

    use super::GamePlayBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(TransformBundle::new())
                .with_bundle(InputBundle::<PlayerAxisControl, PlayerActionControl>::new())
                .with_bundle(UiBundle::<PlayerAxisControl, PlayerActionControl>::new())
                .with_bundle(GamePlayBundle)
                .run()
                .is_ok()
        );
    }
}
