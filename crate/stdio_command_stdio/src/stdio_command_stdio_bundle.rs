use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;
use typename::TypeName;

use crate::StdioCommandEventStdinMapper;

/// Adds a `MapperSystem<StdioCommandEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct StdioCommandStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for StdioCommandStdioBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            MapperSystem::<StdioCommandEventStdinMapper>::new(AppEventVariant::StdioCommand),
            &MapperSystem::<StdioCommandEventStdinMapper>::type_name(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use stdio_spi::VariantAndTokens;

    use super::StdioCommandStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));
        AmethystApplication::blank()
            .with_bundle(StdioCommandStdioBundle::new())
            // kcov-ignore-start
            .with_effect(|world| {
                world.read_resource::<EventChannel<VariantAndTokens>>();
            })
            // kcov-ignore-end
            .run()
    }
}
