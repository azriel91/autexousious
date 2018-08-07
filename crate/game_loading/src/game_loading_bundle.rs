use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use typename::TypeName;

use CharacterSelectionSpawningSystem;
use MapSelectionSpawningSystem;

/// Adds game loading systems to the provided dispatcher.
#[derive(Debug, new)]
pub(crate) struct GameLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameLoadingBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            CharacterSelectionSpawningSystem::new(),
            &CharacterSelectionSpawningSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            MapSelectionSpawningSystem::new(),
            &MapSelectionSpawningSystem::type_name(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test_support::prelude::*;

    use super::GameLoadingBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(GameLoadingBundle)
                .run()
                .is_ok()
        );
    }
}
