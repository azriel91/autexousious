use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use game_mode_selection_model::GameModeIndex;
use menu_model::MenuItemWidgetInputSystem;

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
    pub fn system_names() -> Vec<&'static str> {
        vec![
            any::type_name::<MenuItemWidgetInputSystem<GameModeIndex>>(),
            any::type_name::<GameModeSelectionWidgetUiSystem>(),
            any::type_name::<GameModeSelectionSfxSystem>(),
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
            any::type_name::<MenuItemWidgetInputSystem<GameModeIndex>>(),
            &[],
        ); // kcov-ignore

        builder.add(
            GameModeSelectionWidgetUiSystem::new(),
            any::type_name::<GameModeSelectionWidgetUiSystem>(),
            &[any::type_name::<MenuItemWidgetInputSystem<GameModeIndex>>()],
        ); // kcov-ignore

        builder.add(
            GameModeSelectionSfxSystem::new(),
            any::type_name::<GameModeSelectionSfxSystem>(),
            &[any::type_name::<MenuItemWidgetInputSystem<GameModeIndex>>()],
        ); // kcov-ignore

        Ok(())
    }
}
