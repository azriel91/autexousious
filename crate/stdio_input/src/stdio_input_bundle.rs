use std::any;

use amethyst::{
    core::{bundle::SystemBundle, SystemDesc},
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::{StdinSystem, StdinSystemDesc};

/// Adds the `StdinSystem` to the `World`.
#[derive(Debug, new)]
pub struct StdioInputBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for StdioInputBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            StdinSystemDesc::new().build(world),
            any::type_name::<StdinSystem>(),
            &[],
        );
        Ok(())
    }
}
