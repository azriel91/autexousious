use amethyst::{ecs::prelude::*, prelude::*, shrev::EventChannel};
use application_event::AppEvent;
use application_menu::MenuEvent;
use application_state::{AppState, AppStateBuilder};
use derivative::Derivative;
use derive_new::new;
use game_mode_selection_model::{GameModeSelectionEntityId, GameModeSelectionEvent};
use log::debug;

use crate::GameModeSelectionTrans;

/// `State` where game mode selection takes place.
///
/// This state is not intended to be constructed directly, but through the
/// [`GameModeSelectionStateBuilder`][state_builder].
///
/// [state_builder]: game_mode_selection_state/struct.GameModeSelectionStateBuilder.html
pub type GameModeSelectionState =
    AppState<'static, 'static, GameModeSelectionStateDelegate, GameModeSelectionEntityId>;

/// Builder for a `GameModeSelectionState`.
///
/// `SystemBundle`s to run in the `GameModeSelectionState`'s dispatcher are registered on this
/// builder.
pub type GameModeSelectionStateBuilder =
    AppStateBuilder<'static, 'static, GameModeSelectionStateDelegate, GameModeSelectionEntityId>;

/// Delegate `State` for game mode selection.
///
/// This state is not intended to be used directly, but wrapped in an `AppState`. The
/// `GameModeSelectionState` is an alias with this as a delegate state.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct GameModeSelectionStateDelegate {
    /// ID of the reader for menu events.
    #[new(default)]
    menu_event_reader_id: Option<ReaderId<GameModeSelectionEvent>>,
}

impl GameModeSelectionStateDelegate {
    fn initialize_menu_event_channel(&mut self, world: &mut World) {
        let mut menu_event_channel = EventChannel::<GameModeSelectionEvent>::with_capacity(20);
        let reader_id = menu_event_channel.register_reader();
        self.menu_event_reader_id.get_or_insert(reader_id);

        // Replaces the existing channel, if any.
        world.add_resource(menu_event_channel);
    }

    fn terminate_menu_event_channel(&mut self) {
        // By design there is no function to unregister a reader from an `EventChannel`.
        // Nor is there one to remove a resource from the `World`.

        self.menu_event_reader_id = None;
    }
}

impl State<GameData<'static, 'static>, AppEvent> for GameModeSelectionStateDelegate {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'static, 'static>>) {
        self.initialize_menu_event_channel(&mut data.world);
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'static, 'static>>) {
        self.terminate_menu_event_channel();
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'static, 'static>>,
        event: AppEvent,
    ) -> Trans<GameData<'static, 'static>, AppEvent> {
        if let AppEvent::GameModeSelection(game_mode_selection_event) = event {
            debug!(
                "Received game_mode_selection_event: {:?}",
                game_mode_selection_event
            );
            let mut channel = data
                .world
                .write_resource::<EventChannel<GameModeSelectionEvent>>();
            channel.single_write(game_mode_selection_event);
        }
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'static, 'static>>,
    ) -> Trans<GameData<'static, 'static>, AppEvent> {
        let menu_event_channel = data
            .world
            .read_resource::<EventChannel<GameModeSelectionEvent>>();

        let mut reader_id = self
            .menu_event_reader_id
            .as_mut()
            .expect("Expected menu_event_reader_id to be set");
        match menu_event_channel.read(&mut reader_id).next() {
            Some(event) => match *event {
                MenuEvent::Select(idx) => GameModeSelectionTrans::trans(idx),
                MenuEvent::Close => Trans::Pop,
            },
            None => Trans::None,
        }
    }
}
