use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use derive_new::new;
use game_input::ControllerInput;
use tracker::LastTrackerSystem;
use typename::TypeName;

use crate::{MapSelectionWidgetInputSystem, MapSelectionWidgetUiSystem};

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
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
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

        let controller_input_tracker_system =
            LastTrackerSystem::<ControllerInput>::new(stringify!(game_input::ControllerInput));
        let controller_input_tracker_system_name = controller_input_tracker_system.system_name();

        // This depends on `&ControllerInputUpdateSystem::type_name()`, but since it runs in a
        // separate dispatcher, we have to omit it from here.
        builder.add(
            controller_input_tracker_system,
            &controller_input_tracker_system_name,
            &[&MapSelectionWidgetUiSystem::type_name()],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test::prelude::*;
    use game_input::{GameInputBundle, InputConfig, PlayerActionControl, PlayerAxisControl};

    use super::MapSelectionUiBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
                .with_bundle(GameInputBundle::new(InputConfig::default()))
                .with_bundle(MapSelectionUiBundle::new())
                .run()
                .is_ok()
        );
    }
}
