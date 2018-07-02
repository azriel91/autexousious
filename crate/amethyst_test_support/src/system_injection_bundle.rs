use std::marker::PhantomData;

use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::*;

/// Adds a specified `System` to the dispatcher.
#[derive(Debug, new)]
pub(crate) struct SystemInjectionBundle<'a, 'name, Sys>
where
    Sys: for<'s> System<'s> + Send + 'a,
{
    /// `System` to add to the dispatcher.
    system: Sys,
    /// Name to register the system with.
    system_name: &'name str,
    /// Names of the system dependencies.
    system_dependencies: &'name [&'name str],
    /// Marker for `'a` lifetime.
    #[new(default)]
    system_marker: PhantomData<&'a Sys>,
}

impl<'a, 'b, 'name, Sys> SystemBundle<'a, 'b> for SystemInjectionBundle<'a, 'name, Sys>
where
    Sys: for<'s> System<'s> + Send + 'a,
{
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(self.system, self.system_name, self.system_dependencies);
        Ok(())
    }
}
