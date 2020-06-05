use amethyst::{
    ecs::{Read, SystemData, World, WorldExt},
    shrev::{EventChannel, ReaderId},
    StateData, Trans,
};
use application_input::ApplicationEvent;

use crate::Intercept;

/// Reads `ApplicationEvent`s and programmatically controls the application control flow.
#[derive(Debug, Default)]
pub struct ApplicationEventIntercept {
    /// ID of the reader for application events.
    pub application_event_reader: Option<ReaderId<ApplicationEvent>>,
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
        <Read<'_, EventChannel<ApplicationEvent>> as SystemData<'_>>::setup(world);
        let reader_id = world
            .write_resource::<EventChannel<ApplicationEvent>>()
            .register_reader(); // kcov-ignore

        self.application_event_reader.get_or_insert(reader_id);
    }

    fn handle_application_events<T, E>(&mut self, world: &mut World) -> Option<Trans<T, E>> {
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();

        let mut reader_id = self
            .application_event_reader
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

impl<T, E> Intercept<T, E> for ApplicationEventIntercept
where
    E: Send + Sync + 'static,
{
    fn on_start_begin(&mut self, data: &mut StateData<'_, T>) {
        self.initialize_application_event_reader(data.world);
    }

    fn fixed_update_begin(&mut self, data: &mut StateData<'_, T>) -> Option<Trans<T, E>> {
        self.handle_application_events(data.world)
    }

    fn update_begin(&mut self, data: &mut StateData<'_, T>) -> Option<Trans<T, E>> {
        self.handle_application_events(data.world)
    }

    fn is_transitive(&self) -> bool {
        true
    }
}
