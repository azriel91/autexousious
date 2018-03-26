use amethyst::prelude::*;
use amethyst::shrev::{EventChannel, ReaderId};
use application_input::ApplicationEvent;

use state::Intercept;

/// Reads `ApplicationEvent`s and programmatically controls the application control flow.
#[derive(Debug, Default)]
pub struct ApplicationEventIntercept {
    /// ID of the reader for application events.
    application_event_reader: Option<ReaderId<ApplicationEvent>>,
}

impl ApplicationEventIntercept {
    /// Returns a new ApplicationEventIntercept
    pub fn new() -> Self {
        Default::default()
    }

    fn initialize_application_event_reader(&mut self, world: &mut World) {
        // You can't (don't have to) unregister a reader from an EventChannel in `on_stop();`:
        //
        // > @torkleyy: No need to unregister, it's just two integer values.
        // > @Rhuagh: Just drop the reader id
        let reader_id = world
            .write_resource::<EventChannel<ApplicationEvent>>()
            .register_reader(); // kcov-ignore

        self.application_event_reader.get_or_insert(reader_id);
    }

    fn handle_application_events(&mut self, world: &mut World) -> Option<Trans> {
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();

        let mut reader_id = self.application_event_reader
            .as_mut()
            .expect("Expected reader to be set");
        let mut storage_iterator = app_event_channel.read(&mut reader_id);
        if let Some(&ApplicationEvent::Exit) = storage_iterator.next() {
            return Some(Trans::Quit);
        }

        // TODO: cover this case when there is support for dummy events
        // <https://gitlab.com/azriel91/autexousious/issues/15>
        None // kcov-ignore
    }
}

impl Intercept for ApplicationEventIntercept {
    fn on_start_begin(&mut self, world: &mut World) {
        self.initialize_application_event_reader(world);
    }

    fn fixed_update_begin(&mut self, world: &mut World) -> Option<Trans> {
        self.handle_application_events(world)
    }

    fn update_begin(&mut self, world: &mut World) -> Option<Trans> {
        self.handle_application_events(world)
    }
}

#[cfg(test)]
mod test {
    use std::mem::discriminant;

    use amethyst::prelude::*;
    use amethyst::shrev::EventChannel;
    use application_input::ApplicationEvent;

    use super::ApplicationEventIntercept;
    use state::Intercept;

    fn setup() -> (ApplicationEventIntercept, World) {
        let mut world = World::new();
        world.add_resource(EventChannel::<ApplicationEvent>::with_capacity(10));

        (ApplicationEventIntercept::new(), world)
    }

    #[test]
    fn on_start_begin_initializes_application_event_channel_reader() {
        let (mut intercept, mut world) = setup();

        assert!(intercept.application_event_reader.is_none());

        intercept.on_start_begin(&mut world);

        assert!(intercept.application_event_reader.is_some());
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();
        let mut reader_id = &mut intercept.application_event_reader.as_mut().unwrap();
        assert_eq!(None, app_event_channel.read(&mut reader_id).next());
    }

    #[test]
    fn fixed_update_begin_returns_trans_quit_on_application_event() {
        let (mut intercept, mut world) = setup();

        // register reader
        intercept.on_start_begin(&mut world);

        {
            let mut app_event_channel = world.write_resource::<EventChannel<ApplicationEvent>>();
            app_event_channel.single_write(ApplicationEvent::Exit);
        } // kcov-ignore

        assert_eq!(
            discriminant(&Trans::Quit),
            discriminant(&intercept.fixed_update_begin(&mut world).unwrap())
        );
    }

    #[test]
    fn update_begin_returns_trans_quit_on_application_event() {
        let (mut intercept, mut world) = setup();

        // register reader
        intercept.on_start_begin(&mut world);

        {
            let mut app_event_channel = world.write_resource::<EventChannel<ApplicationEvent>>();
            app_event_channel.single_write(ApplicationEvent::Exit);
        } // kcov-ignore

        assert_eq!(
            discriminant(&Trans::Quit),
            discriminant(&intercept.update_begin(&mut world).unwrap())
        );
    }
}
