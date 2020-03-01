use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;

use crate::SessionJoinEventStdinMapper;

/// Adds a `MapperSystem<SessionJoinEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct SessionJoinStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SessionJoinStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapperSystem::<SessionJoinEventStdinMapper>::new(AppEventVariant::SessionJoin),
            any::type_name::<MapperSystem<SessionJoinEventStdinMapper>>(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
