use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;

use crate::AssetSelectionEventStdinMapper;

/// Adds a `MapperSystem<AssetSelectionEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct AssetSelectionStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for AssetSelectionStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapperSystem::<AssetSelectionEventStdinMapper>::new(AppEventVariant::AssetSelection),
            any::type_name::<MapperSystem<AssetSelectionEventStdinMapper>>(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
