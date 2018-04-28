//! ECS input bundle for custom events

use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::prelude::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;

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
    fn build(self, world: &mut World, _: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        world.add_resource(EventChannel::<ApplicationEvent>::with_capacity(100));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::core::bundle::ECSBundle;
    use amethyst::ecs::prelude::{DispatcherBuilder, World};
    use amethyst::shrev::EventChannel;

    use event::ApplicationEvent;

    use super::ApplicationInputBundle;

    #[test]
    fn build_adds_application_event_channel_to_world() {
        let bundle = ApplicationInputBundle::new();
        let mut world = World::new();
        let mut builder = DispatcherBuilder::new();

        bundle
            .build(&mut world, &mut builder)
            .expect("ApplicationInputBundle#build() should succeed");

        // If the event channel was not registered, the next line will panic
        let _app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();
    }
}
