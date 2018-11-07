use amethyst::{
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType},
};
use application_menu::{MenuEvent, MenuItem};
use game_mode_selection_model::GameModeIndex;

/// System that processes `UiEvent`s and generates `MenuEvent`s.
#[derive(Debug, Default, TypeName)]
pub struct UiEventHandlerSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl UiEventHandlerSystem {
    pub fn new() -> Self {
        Default::default()
    }
}

type UiEventHandlerSystemData<'s> = (
    Read<'s, EventChannel<UiEvent>>,
    ReadStorage<'s, MenuItem<GameModeIndex>>,
    Write<'s, EventChannel<MenuEvent<GameModeIndex>>>,
);

impl<'s> System<'s> for UiEventHandlerSystem {
    type SystemData = UiEventHandlerSystemData<'s>;

    fn run(&mut self, (ui_events, menu_items, mut menu_events): Self::SystemData) {
        for ev in ui_events.read(self.reader_id.as_mut().unwrap()) {
            if let UiEvent {
                event_type: UiEventType::Click,
                target: entity,
            } = *ev
            {
                if let Some(menu_item) = menu_items.get(entity) {
                    let menu_event = MenuEvent::Select(menu_item.index);
                    debug!("Sending menu event: {:?}", &menu_event);
                    menu_events.single_write(menu_event);
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(res.fetch_mut::<EventChannel<UiEvent>>().register_reader());
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        ecs::prelude::*,
        shrev::{EventChannel, ReaderId},
        ui::{UiEvent, UiEventType},
    };
    use amethyst_test::prelude::*;
    use application_menu::{MenuEvent, MenuItem};
    use game_mode_selection_model::GameModeIndex;

    use super::UiEventHandlerSystem;

    fn setup_menu_event_reader(world: &mut World) {
        let menu_event_channel_reader = world
            .write_resource::<EventChannel<MenuEvent<GameModeIndex>>>()
            .register_reader(); // kcov-ignore

        world.add_resource(EffectReturn(menu_event_channel_reader));
    }

    #[test]
    fn run_without_ui_events_does_not_send_menu_event() {
        assert!(AmethystApplication::ui_base::<String, String>()
            .with_system(UiEventHandlerSystem::new(), "", &[])
            .with_setup(setup_menu_event_reader)
            .with_assertion(|world| {
                let mut menu_event_channel_reader = &mut world
                    .write_resource::<EffectReturn<ReaderId<MenuEvent<GameModeIndex>>>>()
                    .0;

                let menu_event_channel =
                    world.read_resource::<EventChannel<MenuEvent<GameModeIndex>>>();
                let mut menu_event_iter = menu_event_channel.read(&mut menu_event_channel_reader);
                assert_eq!(None, menu_event_iter.next());
            })
            .run()
            .is_ok());
    }

    #[test]
    fn run_with_non_click_ui_event_does_not_send_menu_event() {
        assert!(AmethystApplication::ui_base::<String, String>()
            .with_system(UiEventHandlerSystem::new(), "", &[])
            .with_setup(setup_menu_event_reader)
            .with_setup(|world| {
                let entity = world.create_entity().build();
                let mut ui_event_channel = world.write_resource::<EventChannel<UiEvent>>();

                let ui_event_types = vec![
                    UiEventType::ClickStart,
                    UiEventType::ClickStop,
                    UiEventType::HoverStart,
                    UiEventType::HoverStop,
                ];
                let ui_events = ui_event_types
                    .into_iter()
                    .map(|event_type| UiEvent {
                        event_type,
                        target: entity,
                    })
                    .collect::<Vec<UiEvent>>();
                ui_event_channel.iter_write(ui_events.into_iter());
            })
            .with_assertion(|world| {
                let mut menu_event_channel_reader = &mut world
                    .write_resource::<EffectReturn<ReaderId<MenuEvent<GameModeIndex>>>>()
                    .0;

                let menu_event_channel =
                    world.read_resource::<EventChannel<MenuEvent<GameModeIndex>>>();
                let mut menu_event_iter = menu_event_channel.read(&mut menu_event_channel_reader);
                assert_eq!(None, menu_event_iter.next());
            })
            .run()
            .is_ok());
    }

    #[test]
    fn run_with_click_ui_event_sends_select_menu_event() {
        assert!(AmethystApplication::ui_base::<String, String>()
            .with_system(UiEventHandlerSystem::new(), "", &[])
            .with_setup(setup_menu_event_reader)
            .with_setup(|world| {
                let entity = world
                    .create_entity()
                    .with(MenuItem {
                        index: GameModeIndex::StartGame,
                    })
                    .build();

                let mut ui_event_channel = world.write_resource::<EventChannel<UiEvent>>();
                ui_event_channel.single_write(UiEvent {
                    event_type: UiEventType::Click,
                    target: entity,
                });
            })
            .with_assertion(|world| {
                let mut menu_event_channel_reader = &mut world
                    .write_resource::<EffectReturn<ReaderId<MenuEvent<GameModeIndex>>>>()
                    .0;

                let menu_event_channel =
                    world.read_resource::<EventChannel<MenuEvent<GameModeIndex>>>();
                let mut menu_event_iter = menu_event_channel.read(&mut menu_event_channel_reader);
                assert_eq!(
                    Some(&MenuEvent::Select(GameModeIndex::StartGame)),
                    menu_event_iter.next()
                );
                assert_eq!(None, menu_event_iter.next());
            })
            .run()
            .is_ok());
    }

    #[test]
    fn run_with_click_ui_event_on_non_menu_item_does_not_send_menu_event() {
        assert!(AmethystApplication::ui_base::<String, String>()
            .with_system(UiEventHandlerSystem::new(), "", &[])
            .with_setup(setup_menu_event_reader)
            .with_setup(|world| {
                let entity = world.create_entity().build();

                let mut ui_event_channel = world.write_resource::<EventChannel<UiEvent>>();
                ui_event_channel.single_write(UiEvent {
                    event_type: UiEventType::Click,
                    target: entity,
                });
            })
            .with_assertion(|world| {
                let mut menu_event_channel_reader = &mut world
                    .write_resource::<EffectReturn<ReaderId<MenuEvent<GameModeIndex>>>>()
                    .0;

                let menu_event_channel =
                    world.read_resource::<EventChannel<MenuEvent<GameModeIndex>>>();
                let mut menu_event_iter = menu_event_channel.read(&mut menu_event_channel_reader);
                assert_eq!(None, menu_event_iter.next());
            })
            .run()
            .is_ok());
    }
}
