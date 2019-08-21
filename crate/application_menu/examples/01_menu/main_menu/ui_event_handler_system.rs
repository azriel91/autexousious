use amethyst::{
    ecs::prelude::{ReadStorage, System, SystemData, World, Write},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType},
};
use application_menu::{MenuEvent, MenuItem};
use log::{info, trace};

use crate::main_menu;

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
    Write<'s, EventChannel<MenuEvent<main_menu::Index>>>,
    ReadStorage<'s, MenuItem<main_menu::Index>>,
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

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }
}
