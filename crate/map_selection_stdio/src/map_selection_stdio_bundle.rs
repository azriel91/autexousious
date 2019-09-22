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

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{ecs::WorldExt, shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use asset_model::loaded::{AssetIdMappings, AssetTypeMappings};
    use stdio_spi::VariantAndTokens;

    use super::MapSelectionStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::blank()
            .with_bundle(MapSelectionStdioBundle::new())
            .with_effect(|world| {
                world.read_resource::<EventChannel<VariantAndTokens>>();
                world.read_resource::<AssetIdMappings>();
                world.read_resource::<AssetTypeMappings>();
            })
            .run()
    }
}
