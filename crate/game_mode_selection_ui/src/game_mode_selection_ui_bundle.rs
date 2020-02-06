use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::GameModeSelectionSfxSystem;

/// Adds the systems that set up and manage the `GameModeSelectionUi`.
///
/// The `GameInputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct GameModeSelectionUiBundle;

impl GameModeSelectionUiBundle {
    /// Returns the system names added by this bundle.
    ///
    /// This allows consumers to specify the systems as dependencies.
    pub fn system_names() -> Vec<&'static str> {
        vec![any::type_name::<GameModeSelectionSfxSystem>()]
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for GameModeSelectionUiBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            GameModeSelectionSfxSystem::new(),
            any::type_name::<GameModeSelectionSfxSystem>(),
            &[],
        ); // kcov-ignore

        Ok(())
    }
}
