use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;

use crate::NetworkJoinEventStdinMapper;

/// Adds a `MapperSystem<NetworkJoinEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct NetworkJoinStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for NetworkJoinStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapperSystem::<NetworkJoinEventStdinMapper>::new(
                AppEventVariant::NetworkJoin,
            ),
            any::type_name::<MapperSystem<NetworkJoinEventStdinMapper>>(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
