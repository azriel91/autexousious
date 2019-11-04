use amethyst::shrev::{Event, EventChannel, ReaderId};
use log::warn;

/// Additional functions for working with `EventChannel`s.
pub trait EventChannelExt<E> {
    /// Returns the last event from the event channel if any.
    ///
    /// # Parameters
    ///
    /// * `event_rid`: `ReaderId` registered for the event channel.
    fn last_event(&self, event_rid: &mut ReaderId<E>) -> Option<&E>;
}

impl<E> EventChannelExt<E> for EventChannel<E>
where
    E: Event,
{
    fn last_event(&self, event_rid: &mut ReaderId<E>) -> Option<&E> {
        let events_iterator = self.read(event_rid);
        let event_count = events_iterator.len();

        if event_count > 1 {
            warn!(
                "{} events received, only processing the last event.",
                event_count
            );
        }

        events_iterator.skip(event_count.saturating_sub(1)).next()
    }
}
