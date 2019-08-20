use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::{Read, System, SystemData, World},
    shrev::{EventChannel, ReaderId},
};
use application_menu::MenuEvent;
use derive_new::new;
use game_mode_selection_model::{GameModeIndex, GameModeSelectionEvent};
use typename_derive::TypeName;
use ui_audio_model::{config::UiSfxId, loaded::UiSfxMap};

/// Default volume to play sounds at.
const VOLUME: f32 = 1.0;

/// Plays sounds for the game mode selection UI.
#[derive(Debug, Default, TypeName, new)]
pub struct GameModeSelectionSfxSystem {
    /// Reader ID for the `GameModeSelectionEvent` event channel.
    #[new(default)]
    game_mode_selection_event_rid: Option<ReaderId<GameModeSelectionEvent>>,
}

type GameModeSelectionSfxSystemData<'s> = (
    Read<'s, EventChannel<GameModeSelectionEvent>>,
    Read<'s, UiSfxMap>,
    Read<'s, AssetStorage<Source>>,
    Option<Read<'s, Output>>,
);

impl<'s> System<'s> for GameModeSelectionSfxSystem {
    type SystemData = GameModeSelectionSfxSystemData<'s>;

    fn run(
        &mut self,
        (game_mode_selection_ec, ui_sfx_map, source_assets, output): Self::SystemData,
    ) {
        // Make sure we empty the event channel, even if we don't have an output device.
        let events_iterator = game_mode_selection_ec.read(
            self.game_mode_selection_event_rid
                .as_mut()
                .expect("Expected reader ID to exist for GameModeSelectionSfxSystem."),
        );

        if let Some(output) = output {
            events_iterator.for_each(|ev| {
                let ui_sfx_id = match ev {
                    MenuEvent::Select(GameModeIndex::StartGame)
                    | MenuEvent::Select(GameModeIndex::Exit) => Some(UiSfxId::Confirm),
                    MenuEvent::Close => None,
                };

                if let Some(ui_sfx_id) = ui_sfx_id {
                    let ui_sfx = ui_sfx_map
                        .get(&ui_sfx_id)
                        .and_then(|ui_sfx_handle| source_assets.get(ui_sfx_handle));

                    if let Some(ui_sfx) = ui_sfx {
                        output.play_once(ui_sfx, VOLUME);
                    }
                }
            });
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.game_mode_selection_event_rid = Some(
            world
                .fetch_mut::<EventChannel<GameModeSelectionEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_menu::MenuEvent;
    use application_test_support::AutexousiousApplication;
    use game_mode_selection_model::{GameModeIndex, GameModeSelectionEvent};

    use super::GameModeSelectionSfxSystem;

    #[test]
    fn plays_sound_on_select_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(GameModeSelectionSfxSystem::new(), "", &[])
            .with_effect(|world| {
                let event = MenuEvent::Select(GameModeIndex::StartGame);
                send_event(world, event);
            })
            .with_assertion(|_world| {})
            .run_isolated()
    }

    fn send_event(world: &mut World, event: GameModeSelectionEvent) {
        let mut ec = world.write_resource::<EventChannel<GameModeSelectionEvent>>();
        ec.single_write(event)
    } // kcov-ignore
}
