use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
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
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let deps = self
            .system_dependencies
            .as_ref()
            // kcov-ignore-start
            .map_or_else(Vec::new, |deps| {
                deps.iter().map(AsRef::as_ref).collect::<Vec<&str>>()
            });
        // kcov-ignore-end
        builder.add(
            MapperSystem::<ControlInputEventStdinMapper>::new(AppEventVariant::ControlInput),
            &MapperSystem::<ControlInputEventStdinMapper>::type_name(),
            &deps,
        ); // kcov-ignore
        Ok(())
    }
}
