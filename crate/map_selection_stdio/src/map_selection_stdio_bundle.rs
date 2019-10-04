use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;
use typename::TypeName;

use crate::MapSelectionEventStdinMapper;

/// Adds a `MapperSystem<MapSelectionEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct MapSelectionStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MapSelectionStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapperSystem::<MapSelectionEventStdinMapper>::new(AppEventVariant::MapSelection),
            &MapperSystem::<MapSelectionEventStdinMapper>::type_name(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
