#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{World, WorldExt},
        winit::{
            event::{
                DeviceId, ElementState, Event, KeyboardInput, ModifiersState, ScanCode,
                VirtualKeyCode, WindowEvent,
            },
            window::WindowId,
        },
        StateData, StateEvent, Trans,
    };
    use debug_util_amethyst::assert_eq_opt_trans;

    use application_robot::{Intercept, KeyboardEscapeIntercept};

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

    #[allow(deprecated)]
    fn key_event(scancode: ScanCode, virtual_keycode: VirtualKeyCode) -> Event<'static, ()> {
        Event::WindowEvent {
            window_id: unsafe { WindowId::dummy() },
            event: WindowEvent::KeyboardInput {
                is_synthetic: true,
                device_id: unsafe { DeviceId::dummy() },
                input: KeyboardInput {
                    scancode,
                    state: ElementState::Pressed,
                    virtual_keycode: Some(virtual_keycode),
                    modifiers: ModifiersState::default(),
                },
            },
        }
    }
}
