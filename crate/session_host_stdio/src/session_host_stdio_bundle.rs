use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;

use crate::SessionHostEventStdinMapper;

/// Adds a `MapperSystem<SessionHostEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct SessionHostStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SessionHostStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapperSystem::<SessionHostEventStdinMapper>::new(AppEventVariant::SessionHost),
            any::type_name::<MapperSystem<SessionHostEventStdinMapper>>(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
