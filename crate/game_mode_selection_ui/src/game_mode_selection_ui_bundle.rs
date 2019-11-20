use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_menu::MenuItemWidgetInputSystem;
use derive_new::new;
use game_mode_selection_model::GameModeIndex;
use typename::TypeName;

use crate::{GameModeSelectionSfxSystem, GameModeSelectionWidgetUiSystem};

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
        vec![
            MenuItemWidgetInputSystem::<GameModeIndex>::type_name(),
            GameModeSelectionWidgetUiSystem::type_name(),
            GameModeSelectionSfxSystem::type_name(),
        ]
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for GameModeSelectionUiBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MenuItemWidgetInputSystem::<GameModeIndex>::new(),
            &MenuItemWidgetInputSystem::<GameModeIndex>::type_name(),
            &[],
        ); // kcov-ignore

        // builder.add(
        //     GameModeSelectionWidgetUiSystem::new(),
        //     &GameModeSelectionWidgetUiSystem::type_name(),
        //     &[&MenuItemWidgetInputSystem::<GameModeIndex>::type_name()],
        // ); // kcov-ignore

        builder.add(
            GameModeSelectionSfxSystem::new(),
            &GameModeSelectionSfxSystem::type_name(),
            &[&MenuItemWidgetInputSystem::<GameModeIndex>::type_name()],
        ); // kcov-ignore

        Ok(())
    }
}
