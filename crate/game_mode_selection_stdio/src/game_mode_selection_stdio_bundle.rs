use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;

use crate::GameModeSelectionEventStdinMapper;

/// Adds a `MapperSystem<GameModeSelectionEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct GameModeSelectionStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameModeSelectionStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapperSystem::<GameModeSelectionEventStdinMapper>::new(
                AppEventVariant::GameModeSelection,
            ),
            any::type_name::<MapperSystem<GameModeSelectionEventStdinMapper>>(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
