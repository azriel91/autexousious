//! ECS input bundle for custom events

use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::shrev::EventChannel;
use amethyst::ecs::{DispatcherBuilder, World};

use event::ApplicationEvent;

/// Bundle for custom application events.
///
/// Adds an `EventChannel<ApplicationEvent>` to the world. See the [module level documentation]
/// (index.html) for more details.
#[derive(Debug, Default)]
pub struct ApplicationInputBundle;

impl ApplicationInputBundle {
    /// Returns an application bundle.
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a, 'b> ECSBundle<'a, 'b> for ApplicationInputBundle {
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.add_resource(EventChannel::<ApplicationEvent>::with_capacity(100));

        Ok(builder)
    }
}
