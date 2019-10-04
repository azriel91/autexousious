use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use typename::TypeName;

use crate::{
    CharacterSelectionInputSystem, CharacterSelectionSfxSystem,
    CharacterSelectionWidgetInputSystem, CharacterSelectionWidgetUiSystem,
};

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
        vec![
            CharacterSelectionInputSystem::type_name(),
            CharacterSelectionWidgetInputSystem::type_name(),
            CharacterSelectionWidgetUiSystem::type_name(),
            CharacterSelectionSfxSystem::type_name(),
        ]
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterSelectionUiBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // Order this first, as it means we don't transition until attack has been pressed *after*
        // widgets are ready.
        builder.add(
            CharacterSelectionInputSystem::new(),
            &CharacterSelectionInputSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            CharacterSelectionWidgetInputSystem::new(),
            &CharacterSelectionWidgetInputSystem::type_name(),
            &[&CharacterSelectionInputSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            CharacterSelectionWidgetUiSystem::new(),
            &CharacterSelectionWidgetUiSystem::type_name(),
            &[&CharacterSelectionWidgetInputSystem::type_name()],
        ); // kcov-ignore

        builder.add(
            CharacterSelectionSfxSystem::new(),
            &CharacterSelectionSfxSystem::type_name(),
            &[&CharacterSelectionWidgetInputSystem::type_name()],
        ); // kcov-ignore

        Ok(())
    }
}
