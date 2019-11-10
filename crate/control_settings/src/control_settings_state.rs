use amethyst::{
    ecs::WorldExt,
    input::{is_key_down, VirtualKeyCode},
    utils::removal::Removal,
    GameData, State, StateData, Trans,
};
use application_event::AppEvent;
use control_settings_model::{ControlSettingsEntity, ControlSettingsEvent};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use state_registry::StateId;
use state_support::StateEntityUtils;

/// `State` where game play takes place.
#[derive(Derivative, Default, new)]
#[derivative(Debug)]
pub struct ControlSettingsState;

impl<'a, 'b> State<GameData<'a, 'b>, AppEvent> for ControlSettingsState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.register::<Removal<ControlSettingsEntity>>();
        data.world.insert(StateId::ControlSettings);
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        StateEntityUtils::clear::<ControlSettingsEntity>(&mut data.world);
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        data.world.insert(StateId::ControlSettings);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: AppEvent,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        match event {
            AppEvent::Window(window_event) => {
                if is_key_down(&window_event, VirtualKeyCode::Escape) {
                    debug!("Returning from `ControlSettingsState`.");
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            AppEvent::ControlSettings(control_settings_event) => {
                match control_settings_event {
                    ControlSettingsEvent::Return => {
                        debug!("Returning from `ControlSettingsState`.");
                        Trans::Pop
                    }
                    ControlSettingsEvent::ReloadRequest => {
                        // TODO: Reload control settings from file.
                        Trans::None
                    }
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        data.data.update(&data.world);
        Trans::None
    }
}
