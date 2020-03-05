use amethyst::ecs::{Builder, WorldExt};
use asset_model::{loaded::ItemId, play::AssetWorld};
use session_lobby_ui_model::{
    config::SessionLobbyUi,
    loaded::{SessionCodeLabel, SessionDevicesWidget},
};

/// Loads asset items for a `SessionLobbyUi`.
#[derive(Debug)]
pub struct AssetSequenceComponentLoaderUiSessionLobby;

impl AssetSequenceComponentLoaderUiSessionLobby {
    /// Loads asset items for a `SessionLobbyUi`.
    pub fn load(
        asset_world: &mut AssetWorld,
        item_ids_all: &mut Vec<ItemId>,
        session_lobby_ui: &SessionLobbyUi,
    ) {
        let item_id_session_code =
            Self::load_item_entity_session_code(asset_world, session_lobby_ui);
        let item_id_session_devices_widget =
            Self::load_item_entity_session_devices_widget(asset_world, session_lobby_ui);

        item_ids_all.push(item_id_session_code);
        item_ids_all.push(item_id_session_devices_widget);
    }

    fn load_item_entity_session_code(
        asset_world: &mut AssetWorld,
        session_lobby_ui: &SessionLobbyUi,
    ) -> ItemId {
        let ui_label = session_lobby_ui.session_code.clone();
        let item_entity_label = asset_world
            .create_entity()
            .with(SessionCodeLabel)
            .with(ui_label)
            .build();
        ItemId::new(item_entity_label)
    }

    fn load_item_entity_session_devices_widget(
        asset_world: &mut AssetWorld,
        session_lobby_ui: &SessionLobbyUi,
    ) -> ItemId {
        let position_init = session_lobby_ui.session_devices.position;
        let item_entity_session_devices_widget = asset_world
            .create_entity()
            .with(position_init)
            .with(SessionDevicesWidget)
            .build();
        ItemId::new(item_entity_session_devices_widget)
    }
}
