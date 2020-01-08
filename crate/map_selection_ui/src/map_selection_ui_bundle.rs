use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

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
    pub fn system_names() -> Vec<&'static str> {
        vec![
            any::type_name::<MapSelectionWidgetUiSystem>(),
            any::type_name::<MapSelectionWidgetInputSystem>(),
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
            any::type_name::<MapSelectionWidgetInputSystem>(),
            &[],
        ); // kcov-ignore
        builder.add(
            MapSelectionWidgetUiSystem::new(),
            any::type_name::<MapSelectionWidgetUiSystem>(),
            &[any::type_name::<MapSelectionWidgetInputSystem>()],
        ); // kcov-ignore

        builder.add(
            MapSelectionSfxSystem::new(),
            any::type_name::<MapSelectionSfxSystem>(),
            &[any::type_name::<MapSelectionWidgetInputSystem>()],
        ); // kcov-ignore

        Ok(())
    }
}
