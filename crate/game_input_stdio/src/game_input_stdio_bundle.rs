use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;
use typename::TypeName;

use crate::ControlInputEventStdinMapper;

/// Adds a `MapperSystem<ControlInputEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct GameInputStdioBundle {
    /// System names that the `MapperSystem::<ControlInputEventStdinMapper>` should wait on.
    #[new(default)]
    system_dependencies: Option<Vec<String>>,
}

impl GameInputStdioBundle {
    /// Specifies system dependencies for the `MapperSystem::<ControlInputEventStdinMapper>`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: &[String]) -> Self {
        self.system_dependencies = Some(Vec::from(dependencies));
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for GameInputStdioBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let deps = self
            .system_dependencies
            .as_ref()
            .map_or_else(Vec::new, |deps| {
                deps.iter().map(|dep| dep.as_ref()).collect::<Vec<&str>>()
            });
        builder.add(
            MapperSystem::<ControlInputEventStdinMapper>::new(AppEventVariant::ControlInput),
            &MapperSystem::<ControlInputEventStdinMapper>::type_name(),
            &deps,
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{input::InputBundle, renderer::ScreenDimensions, shrev::EventChannel, Error};
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use game_input::GameInputBundle;
    use game_input_model::{PlayerActionControl, PlayerAxisControl};
    use stdio_spi::VariantAndTokens;

    use super::GameInputStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::blank()
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_bundle(InputBundle::<PlayerAxisControl, PlayerActionControl>::new())
            .with_bundle(GameInputBundle::new())
            .with_bundle(GameInputStdioBundle::new())
            // kcov-ignore-start
            .with_effect(|world| {
                world.read_resource::<EventChannel<VariantAndTokens>>();
            })
            // kcov-ignore-end
            .run()
    }
}
