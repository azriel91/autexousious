use amethyst;
use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use amethyst::shrev::{EventChannel, ReaderId};
use application_input::ApplicationEvent;

/// Game mode selection state.
///
/// Available transitions:
///
/// * Selection of game mode
/// * Exiting the application
///
#[derive(Debug, Default)]
pub struct State {
    /// ID of the reader for application events.
    application_event_reader: Option<ReaderId<ApplicationEvent>>,
}

impl State {
    /// Returns a new game mode menu state.
    pub fn new() -> Self {
        Default::default()
    }
}

impl amethyst::State for State {
    fn on_start(&mut self, world: &mut World) {
        // You can't unregister a reader from an EventChannel in on_stop because we don't have to
        //
        // @torkleyy: No need to unregister, it's just two integer values.
        // @Rhuagh: Just drop the reader id
        let reader_id = world
            .write_resource::<EventChannel<ApplicationEvent>>()
            .register_reader();

        self.application_event_reader.get_or_insert(reader_id);
    }

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }
                | WindowEvent::Closed => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn update(&mut self, world: &mut World) -> Trans {
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();

        let mut reader_id = self.application_event_reader
            .as_mut()
            .expect("Expected reader to be set");
        let mut storage_iterator = app_event_channel.read(&mut reader_id);
        while let Some(_event) = storage_iterator.next() {
            return Trans::Quit;
        }

        Trans::None
    }
}

#[cfg(test)]
mod test {
    use amethyst::ecs::World;
    use amethyst::renderer::{Event, WindowEvent};
    use amethyst::shrev::EventChannel;
    use amethyst::State as AmethystState;
    use amethyst::prelude::*;
    use application_input::ApplicationEvent;
    use enigo::{Enigo, Key, KeyboardControllable};
    use winit::{ControlFlow, EventsLoop, Window};

    use super::State;

    fn setup() -> (State, World) {
        let mut world = World::new();
        world.add_resource(EventChannel::<ApplicationEvent>::with_capacity(10));

        (State::new(), world)
    }

    #[test]
    fn on_start_registers_reader() {
        let (mut state, mut world) = setup();

        state.on_start(&mut world);

        assert!(state.application_event_reader.is_some());
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();
        let mut reader_id = &mut state.application_event_reader.as_mut().unwrap();
        assert_eq!(None, app_event_channel.read(&mut reader_id).next());
    }

    #[test]
    fn handle_event_returns_trans_quit_on_escape_keyboard_input() {
        let (mut state, mut world) = setup();

        let mut events_loop = EventsLoop::new();
        let _window = Window::new(&events_loop).unwrap();

        let mut enigo = Enigo::new();
        enigo.key_click(Key::Escape);

        events_loop.run_forever(|event| match event {
            Event::WindowEvent {
                event: ref window_event,
                ..
            } => match window_event {
                &WindowEvent::KeyboardInput { .. } => {
                    if let Trans::Quit = state.handle_event(&mut world, event.clone()) {
                        ControlFlow::Break
                    } else {
                        panic!("Expected Trans::Quit") // kcov-ignore
                    }
                }
                _ => ControlFlow::Continue,
            },
            _ => ControlFlow::Continue,
        });
    }
}
