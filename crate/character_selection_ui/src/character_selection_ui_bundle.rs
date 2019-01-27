use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use derive_new::new;
use game_input::ControllerInput;
use tracker::LastTrackerSystem;
use typename::TypeName;

use crate::{CharacterSelectionWidgetInputSystem, CharacterSelectionWidgetUiSystem};

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
            CharacterSelectionWidgetUiSystem::type_name(),
            CharacterSelectionWidgetInputSystem::type_name(),
        ]
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterSelectionUiBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            CharacterSelectionWidgetInputSystem::new(),
            &CharacterSelectionWidgetInputSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            CharacterSelectionWidgetUiSystem::new(),
            &CharacterSelectionWidgetUiSystem::type_name(),
            &[&CharacterSelectionWidgetInputSystem::type_name()],
        ); // kcov-ignore

        let controller_input_tracker_system =
            LastTrackerSystem::<ControllerInput>::new(stringify!(game_input::ControllerInput));
        let controller_input_tracker_system_name = controller_input_tracker_system.system_name();

        // This depends on `&ControllerInputUpdateSystem::type_name()`, but since it runs in a
        // separate dispatcher, we have to omit it from here.
        builder.add(
            controller_input_tracker_system,
            &controller_input_tracker_system_name,
            &[&CharacterSelectionWidgetUiSystem::type_name()],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test::prelude::*;
    use game_input::{GameInputBundle, InputConfig, PlayerActionControl, PlayerAxisControl};

    use super::CharacterSelectionUiBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
                .with_bundle(GameInputBundle::new(InputConfig::default()))
                .with_bundle(CharacterSelectionUiBundle::new())
                .run()
                .is_ok()
        );
    }
}
