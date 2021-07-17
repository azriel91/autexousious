use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    StateData, StateEvent, Trans,
};

use crate::Intercept;

/// Exits when the `Escape` key is pressed.
#[derive(Clone, Debug, Default)]
pub struct KeyboardEscapeIntercept;

impl<T> Intercept<T, StateEvent> for KeyboardEscapeIntercept {
    fn handle_event_begin(
        &mut self,
        _data: &mut StateData<'_, T>,
        event: &mut StateEvent,
    ) -> Option<Trans<T, StateEvent>> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(event) || is_key_down(event, VirtualKeyCode::Escape) {
                Some(Trans::Quit)
            } else {
                None
            }
        } else {
            // TODO: cover this case when there is support for dummy events #15
            None // kcov-ignore
        }
    }
}
