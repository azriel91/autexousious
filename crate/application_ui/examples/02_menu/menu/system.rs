use amethyst::ecs::{FetchMut, ReadStorage, System};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{UiEvent, UiEventType};

use menu::event::MenuEvent;
use menu::main_menu;
use menu::menu_item::MenuItem;

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

impl<'s> System<'s> for UiEventHandlerSystem {
    type SystemData = (
        FetchMut<'s, EventChannel<UiEvent>>,
        FetchMut<'s, EventChannel<MenuEvent<main_menu::Index>>>,
        ReadStorage<'s, MenuItem<main_menu::Index>>,
    );

    fn run(&mut self, (mut ui_events, mut menu_events, menu_items): Self::SystemData) {
        if self.reader_id.is_none() {
            self.reader_id = Some(ui_events.register_reader());
        }
        for ev in ui_events.read(self.reader_id.as_mut().unwrap()) {
            match ev {
                &UiEvent {
                    event_type: UiEventType::Click,
                    target: entity,
                } => {
                    if let Some(menu_item) = menu_items.get(entity) {
                        let menu_event = MenuEvent::Select(menu_item.index);
                        info!("Sending menu event: {:?}", &menu_event);
                        menu_events.single_write(menu_event);
                    } else {
                        trace!("Non-menu-item entity clicked: {:?}", entity)
                    }
                }
                _ => {}
            }
        }
    }
}
