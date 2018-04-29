use amethyst::ecs::prelude::{ReadStorage, Resources, System, SystemData, Write};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{UiEvent, UiEventType};
use application_menu::{MenuEvent, MenuItem};

use index::Index;

/// System that processes `UiEvent`s and generates `MenuEvent`s.
#[derive(Debug, Default)]
pub struct UiEventHandlerSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl UiEventHandlerSystem {
    pub fn new() -> Self {
        Default::default()
    }
}

type UiEventHandlerSystemData<'s> = (
    Write<'s, EventChannel<UiEvent>>,
    Write<'s, EventChannel<MenuEvent<Index>>>,
    ReadStorage<'s, MenuItem<Index>>,
);

impl<'s> System<'s> for UiEventHandlerSystem {
    type SystemData = UiEventHandlerSystemData<'s>;

    fn run(&mut self, (mut ui_events, mut menu_events, menu_items): Self::SystemData) {
        if self.reader_id.is_none() {
            self.reader_id = Some(ui_events.register_reader());
        }
        for ev in ui_events.read(self.reader_id.as_mut().unwrap()) {
            if let UiEvent {
                event_type: UiEventType::Click,
                target: entity,
            } = *ev
            {
                if let Some(menu_item) = menu_items.get(entity) {
                    let menu_event = MenuEvent::Select(menu_item.index);
                    info!("Sending menu event: {:?}", &menu_event);
                    menu_events.single_write(menu_event);
                } else {
                    trace!("Non-menu-item entity clicked: {:?}", entity)
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use amethyst::ecs::prelude::World;
    use amethyst::shred::ParSeq;
    use amethyst::shrev::{EventChannel, ReaderId};
    use amethyst::ui::{UiEvent, UiEventType};
    use application_menu::{MenuEvent, MenuItem};
    use rayon::{ThreadPool, ThreadPoolBuilder};

    use super::UiEventHandlerSystem;
    use index::Index;

    fn setup() -> (
        ParSeq<Arc<ThreadPool>, UiEventHandlerSystem>,
        World,
        ReaderId<MenuEvent<Index>>,
    ) {
        let mut world = World::new();
        world.add_resource(EventChannel::<MenuEvent<Index>>::with_capacity(10));
        world.add_resource(EventChannel::<UiEvent>::with_capacity(10)); // needed by system
        world.register::<MenuItem<Index>>();

        let menu_event_channel_reader = {
            let mut menu_event_channel = world.write_resource::<EventChannel<MenuEvent<Index>>>();
            menu_event_channel.register_reader()
        }; // kcov-ignore

        let mut dispatcher = ParSeq::new(
            UiEventHandlerSystem::new(),
            Arc::new(ThreadPoolBuilder::new().build().unwrap()),
        );
        dispatcher.dispatch(&mut world.res); // Get system to register reader ID

        (dispatcher, world, menu_event_channel_reader)
    }

    #[test]
    fn run_without_ui_events_does_not_send_menu_event() {
        let (mut dispatcher, mut world, mut menu_event_channel_reader) = setup();

        // We don't write any UI events

        dispatcher.dispatch(&mut world.res);

        let menu_event_channel = world.read_resource::<EventChannel<MenuEvent<Index>>>();
        let mut menu_event_iter = menu_event_channel.read(&mut menu_event_channel_reader);
        assert_eq!(None, menu_event_iter.next());
    }

    #[test]
    fn run_with_non_click_ui_event_does_not_send_menu_event() {
        let (mut dispatcher, mut world, mut menu_event_channel_reader) = setup();

        {
            let entity = world.create_entity().build();
            let mut ui_event_channel = world.write_resource::<EventChannel<UiEvent>>();

            let mut ui_event_types = vec![
                UiEventType::ClickStart,
                UiEventType::ClickStop,
                UiEventType::HoverStart,
                UiEventType::HoverStop,
            ];
            let ui_events = ui_event_types
                .drain(..)
                .map(|event_type| UiEvent {
                    event_type,
                    target: entity,
                })
                .collect::<Vec<UiEvent>>();
            ui_event_channel.iter_write(ui_events.into_iter());
        }

        dispatcher.dispatch(&mut world.res);

        let menu_event_channel = world.read_resource::<EventChannel<MenuEvent<Index>>>();
        let mut menu_event_iter = menu_event_channel.read(&mut menu_event_channel_reader);
        assert_eq!(None, menu_event_iter.next());
    }

    #[test]
    fn run_with_click_ui_event_sends_select_menu_event() {
        let (mut dispatcher, mut world, mut menu_event_channel_reader) = setup();

        {
            let entity = world
                .create_entity()
                .with(MenuItem {
                    index: Index::StartGame,
                })
                .build();

            let mut ui_event_channel = world.write_resource::<EventChannel<UiEvent>>();
            ui_event_channel.single_write(UiEvent {
                event_type: UiEventType::Click,
                target: entity,
            });
        } // kcov-ignore

        dispatcher.dispatch(&mut world.res);

        let menu_event_channel = world.read_resource::<EventChannel<MenuEvent<Index>>>();
        let mut menu_event_iter = menu_event_channel.read(&mut menu_event_channel_reader);
        assert_eq!(
            Some(&MenuEvent::Select(Index::StartGame)),
            menu_event_iter.next()
        );
        assert_eq!(None, menu_event_iter.next());
    }

    #[test]
    fn run_with_click_ui_event_on_non_menu_item_does_not_send_menu_event() {
        let (mut dispatcher, mut world, mut menu_event_channel_reader) = setup();

        {
            let entity = world.create_entity().build();

            let mut ui_event_channel = world.write_resource::<EventChannel<UiEvent>>();
            ui_event_channel.single_write(UiEvent {
                event_type: UiEventType::Click,
                target: entity,
            });
        } // kcov-ignore

        dispatcher.dispatch(&mut world.res);

        let menu_event_channel = world.read_resource::<EventChannel<MenuEvent<Index>>>();
        let mut menu_event_iter = menu_event_channel.read(&mut menu_event_channel_reader);
        assert_eq!(None, menu_event_iter.next());
    }
}
