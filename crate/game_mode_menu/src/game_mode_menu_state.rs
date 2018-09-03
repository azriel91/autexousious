use amethyst::{
    core::SystemBundle,
    ecs::prelude::*,
    prelude::*,
    shrev::{EventChannel, ReaderId},
};
use application_menu::MenuEvent;

use GameModeMenuBundle;
use Index;
use MenuBuildFn;

/// Game mode selection state.
///
/// Available transitions:
///
/// * Select game mode.
/// * Exit application.
#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct GameModeMenuState {
    /// State specific dispatcher.
    #[derivative(Debug = "ignore")]
    dispatcher: Option<Dispatcher<'static, 'static>>,
    /// Function used to build the menu.
    menu_build_fn: MenuBuildFn,
    /// Menu item entities, which we create / delete when the state is run / paused
    menu_items: Vec<Entity>,
    /// ID of the reader for menu events.
    menu_event_reader: Option<ReaderId<MenuEvent<Index>>>,
}

impl GameModeMenuState {
    /// Returns a new game mode menu state.
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(test)]
    fn internal_new(menu_build_fn: MenuBuildFn) -> Self {
        GameModeMenuState {
            menu_build_fn,
            ..Default::default()
        } // kcov-ignore
    }

    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();

        GameModeMenuBundle::new()
            .build(&mut dispatcher_builder)
            .expect("Failed to register `GameModeMenuBundle`.");

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut world.res);
        self.dispatcher = Some(dispatcher);
    }

    fn terminate_dispatcher(&mut self) {
        self.dispatcher = None;
    }

    fn initialize_menu_event_channel(&mut self, world: &mut World) {
        let mut menu_event_channel = EventChannel::<MenuEvent<Index>>::with_capacity(20);
        let reader_id = menu_event_channel.register_reader();
        self.menu_event_reader.get_or_insert(reader_id);

        world.add_resource(menu_event_channel);
    }

    fn terminate_menu_event_channel(&mut self, _world: &mut World) {
        // By design there is no function to unregister a reader from an `EventChannel`.
        // Nor is there one to remove a resource from the `World`.

        self.menu_event_reader.take();
    }

    fn initialize_menu_items(&mut self, world: &mut World) {
        // https://github.com/rust-lang/rust/issues/26186
        // https://stackoverflow.com/q/46472082/1576773
        (&mut *self.menu_build_fn)(world, &mut self.menu_items);
    }

    fn terminate_menu_items(&mut self, world: &mut World) {
        self.menu_items.drain(..).for_each(|menu_item| {
            world
                .delete_entity(menu_item)
                .expect("Failed to delete menu item.");
        });
    }
}

impl<E> State<GameData<'static, 'static>, E> for GameModeMenuState
where
    E: Send + Sync + 'static,
{
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.initialize_dispatcher(&mut data.world);
        self.initialize_menu_event_channel(&mut data.world);
        self.initialize_menu_items(&mut data.world);
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.terminate_menu_items(&mut data.world);
        self.terminate_menu_event_channel(&mut data.world);
        self.terminate_dispatcher();
    }

    // Need to explicitly hide and show the menu items during pause and resume
    fn on_resume(&mut self, mut data: StateData<GameData>) {
        self.initialize_menu_items(&mut data.world);
    }

    fn on_pause(&mut self, mut data: StateData<GameData>) {
        self.terminate_menu_items(&mut data.world);
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'static, 'static>, E> {
        data.data.update(&data.world);
        self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);

        let menu_event_channel = data.world.read_resource::<EventChannel<MenuEvent<Index>>>();

        let mut reader_id = self
            .menu_event_reader
            .as_mut()
            .expect("Expected menu_event_reader to be set");
        let mut storage_iterator = menu_event_channel.read(&mut reader_id);
        match storage_iterator.next() {
            Some(event) => match *event {
                MenuEvent::Select(idx) => idx.trans(),
                MenuEvent::Close => Trans::Quit,
            },
            None => Trans::None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem::discriminant;
    use std::sync::Arc;

    use amethyst::{ecs::prelude::World, prelude::*, shrev::EventChannel, ui::UiEvent};
    use application_menu::{MenuEvent, MenuItem};
    use rayon::ThreadPoolBuilder;

    use super::GameModeMenuState;
    use Index;
    use MenuBuildFn;

    fn setup<'a, 'b>() -> (GameModeMenuState, World, GameData<'a, 'b>) {
        setup_with_menu_items(MenuBuildFn(Box::new(|_, _| {})))
    }

    fn setup_with_menu_items<'a, 'b>(
        menu_build_fn: MenuBuildFn,
    ) -> (GameModeMenuState, World, GameData<'a, 'b>) {
        let mut world = World::new();
        world.add_resource(Arc::new(ThreadPoolBuilder::new().build().unwrap()));
        world.add_resource(EventChannel::<MenuEvent<Index>>::with_capacity(10));
        world.add_resource(EventChannel::<UiEvent>::with_capacity(10)); // needed by system
        world.register::<MenuItem<Index>>();

        let game_data = GameDataBuilder::default().build(&mut world);

        (
            GameModeMenuState::internal_new(menu_build_fn),
            world,
            game_data,
        )
    }

    #[test]
    fn on_start_initializes_dispatcher() {
        let (mut state, mut world, mut data) = setup();

        assert!(state.dispatcher.is_none());

        <State<_, ()>>::on_start(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert!(state.dispatcher.is_some());
    }

    #[test]
    fn on_start_initializes_menu_event_channel_reader() {
        let (mut state, mut world, mut data) = setup();

        assert!(state.menu_event_reader.is_none());

        <State<_, ()>>::on_start(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert!(state.menu_event_reader.is_some());
        let menu_event_channel = world.read_resource::<EventChannel<MenuEvent<Index>>>();
        let mut reader_id = &mut state.menu_event_reader.as_mut().unwrap();
        assert_eq!(None, menu_event_channel.read(&mut reader_id).next());
    }

    #[test]
    fn on_start_initializes_menu_items() {
        let (mut state, mut world, mut data) =
            setup_with_menu_items(MenuBuildFn(Box::new(|world, menu_items| {
                menu_items.push(world.create_entity().build())
            })));

        assert!(state.menu_items.is_empty());

        <State<_, ()>>::on_start(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert_eq!(1, state.menu_items.len());
    }

    #[test]
    fn on_stop_terminates_dispatcher() {
        let (mut state, mut world, mut data) = setup();

        <State<_, ()>>::on_start(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert!(state.dispatcher.is_some());

        <State<_, ()>>::on_stop(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert!(state.dispatcher.is_none());
    }

    #[test]
    fn on_stop_terminates_menu_event_channel_reader() {
        let (mut state, mut world, mut data) = setup();

        <State<_, ()>>::on_start(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert!(state.menu_event_reader.is_some());

        <State<_, ()>>::on_stop(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert!(state.menu_event_reader.is_none());
    }

    #[test]
    fn on_stop_terminates_menu_items() {
        let (mut state, mut world, mut data) =
            setup_with_menu_items(MenuBuildFn(Box::new(|world, menu_items| {
                menu_items.push(world.create_entity().build())
            })));

        <State<_, ()>>::on_start(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert_eq!(1, state.menu_items.len());

        <State<_, ()>>::on_stop(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert!(state.menu_items.is_empty());
    }

    #[test]
    fn on_pause_terminates_menu_items() {
        let (mut state, mut world, mut data) =
            setup_with_menu_items(MenuBuildFn(Box::new(|world, menu_items| {
                menu_items.push(world.create_entity().build())
            })));

        <State<_, ()>>::on_start(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert_eq!(1, state.menu_items.len());

        <State<_, ()>>::on_pause(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert!(state.menu_items.is_empty());
    }

    #[test]
    fn on_resume_initializes_menu_items() {
        let (mut state, mut world, mut data) =
            setup_with_menu_items(MenuBuildFn(Box::new(|world, menu_items| {
                menu_items.push(world.create_entity().build())
            })));

        <State<_, ()>>::on_start(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert_eq!(1, state.menu_items.len());

        <State<_, ()>>::on_pause(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert!(state.menu_items.is_empty());

        <State<_, ()>>::on_resume(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert_eq!(1, state.menu_items.len());
    }

    #[test]
    fn update_returns_trans_none_when_no_application_or_menu_event_exists() {
        let (mut state, mut world, mut data) = setup();

        // register reader
        <State<_, ()>>::on_start(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        assert_eq!(
            discriminant(&Trans::None as &Trans<_, ()>),
            discriminant(&state.update(StateData {
                world: &mut world,
                data: &mut data,
            }))
        );
    }

    #[test]
    fn update_returns_trans_quit_on_close_menu_event() {
        let (mut state, mut world, mut data) = setup();

        // register reader
        <State<_, ()>>::on_start(
            &mut state,
            StateData {
                world: &mut world,
                data: &mut data,
            },
        );

        {
            let mut menu_event_channel = world.write_resource::<EventChannel<MenuEvent<Index>>>();
            menu_event_channel.single_write(MenuEvent::Close);
        } // kcov-ignore

        assert_eq!(
            discriminant(&Trans::Quit as &Trans<_, ()>),
            discriminant(&state.update(StateData {
                world: &mut world,
                data: &mut data,
            }))
        );
    }
}
