use amethyst::ecs::{Builder, WorldExt};
use asset_model::{loaded::ItemId, play::AssetWorld};
use session_lobby_ui_model::{
    config::{SessionDeviceWidgetTemplate, SessionLobbyUi},
    loaded::{SessionCodeLabel, SessionDevicesWidget},
};

/// Loads asset items for a `SessionLobbyUi`.
#[derive(Debug)]
pub struct UiAsclSessionLobby;

impl UiAsclSessionLobby {
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
        let session_lobby_ui_model::config::SessionDevicesWidget {
            position: position_init,
            session_device_widget_template:
                SessionDeviceWidgetTemplate {
                    dimensions,
                    device_id,
                    device_name,
                },
        } = session_lobby_ui.session_devices.clone();

        let item_id_session_device_id = ItemId::new(
            asset_world
                .create_entity()
                .with(device_id.position)
                .with(device_id)
                .build(),
        );
        let item_id_session_device_name = ItemId::new(
            asset_world
                .create_entity()
                .with(device_name.position)
                .with(device_name)
                .build(),
        );

        let item_entity_session_devices_widget = asset_world
            .create_entity()
            .with(position_init)
            .with(dimensions)
            .with(SessionDevicesWidget::new(
                item_id_session_device_id,
                item_id_session_device_name,
            ))
            .build();
        ItemId::new(item_entity_session_devices_widget)
    }
}
