use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use derive_new::new;
use typename::TypeName;

use crate::UiEventHandlerSystem;

/// Adds the systems that set up and manage the `GameModeSelectionUi`.
///
/// The `GameInputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct GameModeSelectionUiBundle;

impl GameModeSelectionUiBundle {
    /// Returns the system names added by this bundle.
    ///
    /// This allows consumers to specify the systems as dependencies.
    pub fn system_names() -> Vec<String> {
        vec![UiEventHandlerSystem::type_name()]
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for GameModeSelectionUiBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            UiEventHandlerSystem::new(),
            &UiEventHandlerSystem::type_name(),
            &[],
        ); // kcov-ignore

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test::prelude::*;

    use super::GameModeSelectionUiBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_bundle(GameModeSelectionUiBundle::new())
                .run()
                .is_ok()
        );
    }
}
