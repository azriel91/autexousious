use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::CameraCreator;

/// Adds the following resources to the `World`:
///
/// * `Camera`
#[derive(Debug, new)]
pub struct CameraPlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CameraPlayBundle {
    fn build(
        self,
        world: &mut World,
        _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        CameraCreator::create_in_world(world);

        Ok(())
    }
}
