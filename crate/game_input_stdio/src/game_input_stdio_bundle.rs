use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;

use crate::ControlInputEventStdinMapper;

/// Adds a `MapperSystem<ControlInputEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct GameInputStdioBundle {
    /// System names that the `MapperSystem::<ControlInputEventStdinMapper>` should wait on.
    #[new(default)]
    system_dependencies: Option<Vec<&'static str>>,
}

impl GameInputStdioBundle {
    /// Specifies system dependencies for the `MapperSystem::<ControlInputEventStdinMapper>`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: Vec<&'static str>) -> Self {
        self.system_dependencies = Some(dependencies);
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for GameInputStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let deps = self.system_dependencies.unwrap_or_else(Vec::new);
        builder.add(
            MapperSystem::<ControlInputEventStdinMapper>::new(AppEventVariant::ControlInput),
            any::type_name::<MapperSystem<ControlInputEventStdinMapper>>(),
            &deps,
        ); // kcov-ignore
        Ok(())
    }
}
