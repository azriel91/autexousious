use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    StateData, StateEvent, Trans,
};

use crate::state::Intercept;

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
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
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

#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{World, WorldExt},
        StateData, StateEvent, Trans,
    };
    use debug_util_amethyst::assert_eq_opt_trans;
    use winit::{
        DeviceId, ElementState, Event, KeyboardInput, ModifiersState, ScanCode, VirtualKeyCode,
        WindowEvent, WindowId,
    };

    use super::KeyboardEscapeIntercept;
    use crate::state::Intercept;

    // Development Note: See content of this file at revision `f3fc60f` if you attempt to use enigo.

    #[test]
    fn handle_event_begin_returns_none_on_non_quit_key() {
        let mut intercept = KeyboardEscapeIntercept;
        let mut world = World::new();
        let world = &mut world;

        let event = key_event(0x15, VirtualKeyCode::Back);

        assert_eq_opt_trans(
            None,
            intercept
                .handle_event_begin(
                    &mut StateData {
                        world,
                        data: &mut (),
                    },
                    &mut StateEvent::Window(event),
                )
                .as_ref(),
        ); // kcov-ignore
    }

    #[test]
    fn handle_event_begin_returns_trans_quit_on_escape_key() {
        let mut intercept = KeyboardEscapeIntercept;
        let mut world = World::new();
        let world = &mut world;

        let event = key_event(0x1, VirtualKeyCode::Escape);

        assert_eq_opt_trans(
            Some(&Trans::Quit),
            intercept
                .handle_event_begin(
                    &mut StateData {
                        world,
                        data: &mut (),
                    },
                    &mut StateEvent::Window(event),
                )
                .as_ref(),
        ); // kcov-ignore
    }

    fn key_event(scancode: ScanCode, virtual_keycode: VirtualKeyCode) -> Event {
        Event::WindowEvent {
            window_id: unsafe { WindowId::dummy() },
            event: WindowEvent::KeyboardInput {
                device_id: unsafe { DeviceId::dummy() },
                input: KeyboardInput {
                    scancode,
                    state: ElementState::Pressed,
                    virtual_keycode: Some(virtual_keycode),
                    modifiers: ModifiersState {
                        shift: false,
                        ctrl: false,
                        alt: false,
                        logo: false,
                    },
                },
            },
        }
    }
}
