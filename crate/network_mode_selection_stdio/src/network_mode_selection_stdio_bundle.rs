use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;

use crate::NetworkModeSelectionEventStdinMapper;

/// Adds a `MapperSystem<NetworkModeSelectionEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct NetworkModeSelectionStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for NetworkModeSelectionStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapperSystem::<NetworkModeSelectionEventStdinMapper>::new(
                AppEventVariant::NetworkModeSelection,
            ),
            any::type_name::<MapperSystem<NetworkModeSelectionEventStdinMapper>>(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
