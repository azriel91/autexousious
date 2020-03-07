use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::UiText,
};
use derivative::Derivative;
use derive_new::new;
use network_session_model::play::SessionDevices;
use session_lobby_ui_model::play::{SessionDeviceWidget, SessionDevicesEntities};

/// Updates the text in each `SessionDeviceWidget` with `SessionDevice` ID and name.
#[derive(Debug, new)]
pub struct SessionDeviceWidgetUpdateSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionDeviceWidgetUpdateSystemData<'s> {
    /// `SessionDevicesEntities` resource.
    #[derivative(Debug = "ignore")]
    pub session_devices_entities: Read<'s, SessionDevicesEntities>,
    /// `SessionDevices` resource.
    #[derivative(Debug = "ignore")]
    pub session_devices: Read<'s, SessionDevices>,
    /// `SessionDeviceWidget` components.
    #[derivative(Debug = "ignore")]
    pub session_device_widgets: ReadStorage<'s, SessionDeviceWidget>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
}

impl<'s> System<'s> for SessionDeviceWidgetUpdateSystem {
    type SystemData = SessionDeviceWidgetUpdateSystemData<'s>;

    fn run(
        &mut self,
        SessionDeviceWidgetUpdateSystemData {
            session_devices_entities,
            session_devices,
            session_device_widgets,
            mut ui_texts,
        }: Self::SystemData,
    ) {
        let SessionDevicesEntities {
            session_device_entities,
            ..
        } = &*session_devices_entities;

        if session_devices.len() == session_device_entities.len() {
            // Update values if necessary.
            session_devices
                .iter()
                .zip(
                    session_device_entities
                        .iter()
                        .copied()
                        .map(|session_device_entity| {
                            session_device_widgets.get(session_device_entity).copied()
                        }),
                )
                .for_each(|(session_device, session_device_widget)| {
                    if let Some(session_device_widget) = session_device_widget {
                        let SessionDeviceWidget {
                            entity_id,
                            entity_name,
                        } = session_device_widget;

                        if let Some(ui_text_id) = ui_texts.get_mut(entity_id) {
                            ui_text_id.text = format!("#{}", session_device.id);
                        }
                        if let Some(ui_text_name) = ui_texts.get_mut(entity_name) {
                            if ui_text_name.text != session_device.name.0 {
                                ui_text_name.text = session_device.name.0.clone();
                            }
                        }
                    }
                });
        }
    }
}
