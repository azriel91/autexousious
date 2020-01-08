use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::StdinSystem;

/// Adds the `StdinSystem` to the `World`.
#[derive(Debug, new)]
pub struct StdioInputBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for StdioInputBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(StdinSystem::new(), any::type_name::<StdinSystem>(), &[]);
        Ok(())
    }
}
