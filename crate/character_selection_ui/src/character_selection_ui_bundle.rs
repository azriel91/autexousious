use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use typename::TypeName;

use crate::CharacterSelectionSfxSystem;

/// Adds the systems that set up and manage the `CharacterSelectionUi`.
///
/// The `GameInputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct CharacterSelectionUiBundle;

impl CharacterSelectionUiBundle {
    /// Returns the system names added by this bundle.
    ///
    /// This allows consumers to specify the systems as dependencies.
    pub fn system_names() -> Vec<String> {
        vec![CharacterSelectionSfxSystem::type_name()]
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterSelectionUiBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            CharacterSelectionSfxSystem::new(),
            &CharacterSelectionSfxSystem::type_name(),
            &[],
        ); // kcov-ignore

        Ok(())
    }
}
