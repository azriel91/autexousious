use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use typename::TypeName;

use crate::{MapSelectionSfxSystem, MapSelectionWidgetInputSystem, MapSelectionWidgetUiSystem};

/// Adds the systems that set up and manage the `MapSelectionUi`.
///
/// The `GameInputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct MapSelectionUiBundle;

impl MapSelectionUiBundle {
    /// Returns the system names added by this bundle.
    ///
    /// This allows consumers to specify the systems as dependencies.
    pub fn system_names() -> Vec<String> {
        vec![
            MapSelectionWidgetUiSystem::type_name(),
            MapSelectionWidgetInputSystem::type_name(),
        ]
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for MapSelectionUiBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapSelectionWidgetInputSystem::new(),
            &MapSelectionWidgetInputSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            MapSelectionWidgetUiSystem::new(),
            &MapSelectionWidgetUiSystem::type_name(),
            &[&MapSelectionWidgetInputSystem::type_name()],
        ); // kcov-ignore

        builder.add(
            MapSelectionSfxSystem::new(),
            &MapSelectionSfxSystem::type_name(),
            &[&MapSelectionWidgetInputSystem::type_name()],
        ); // kcov-ignore

        Ok(())
    }
}
