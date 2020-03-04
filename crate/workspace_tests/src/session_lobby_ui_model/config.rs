#[cfg(test)]
mod test {
    use amethyst::ui::Anchor;
    use kinematic_model::config::PositionInit;
    use serde_yaml;
    use ui_label_model::config::UiLabel;
    use ui_model_spi::config::Dimensions;

    use session_lobby_ui_model::config::{
        SessionDeviceWidgetTemplate, SessionDevicesWidget, SessionLobbyUi,
    };

    const SESSION_LOBBY_UI_YAML_ALL: &str = r#"
session_code:
  position: { x: 400, y: 500, z: 11 }
  dimensions: { w: 300, h: 100 }
  align: "Middle"
  font_colour: [0.6, 0.7, 1.0, 1.0]
  font_size: 99

session_devices:
  position: { x: 50, y: 400, z: 11 }

  session_device_widget_template:
    dimensions: { w: 300, h: 40 }

    device_id:
      position   : { x: 0, y: 5 }
      dimensions : { w: 30, h: 30 }
      align      : "BottomLeft"
      font_colour: [0.7, 0.7, 0.7, 1.0]
      font_size  : 30

    device_name:
      position   : { x: 30, y: 5 }
      dimensions : { w: 270, h: 30 }
      align      : "BottomLeft"
      font_colour: [1.0, 1.0, 1.0, 1.0]
      font_size  : 30
"#;

    #[test]
    fn deserialize_session_lobby_ui_yaml_all() {
        let session_lobby_ui = serde_yaml::from_str::<SessionLobbyUi>(SESSION_LOBBY_UI_YAML_ALL)
            .expect("Failed to deserialize `SessionLobbyUi`.");

        let session_code = UiLabel {
            position: PositionInit {
                x: 400,
                y: 500,
                z: 11,
            },
            dimensions: Dimensions { w: 300, h: 100 },
            align: Anchor::Middle,
            font_colour: [0.6, 0.7, 1.0, 1.0],
            font_size: 99,
            ..Default::default()
        };

        let device_id = UiLabel {
            position: PositionInit { x: 0, y: 5, z: 0 },
            dimensions: Dimensions { w: 30, h: 30 },
            align: Anchor::BottomLeft,
            font_colour: [0.7, 0.7, 0.7, 1.0],
            font_size: 30,
            ..Default::default()
        };

        let device_name = UiLabel {
            position: PositionInit { x: 30, y: 5, z: 0 },
            dimensions: Dimensions { w: 270, h: 30 },
            align: Anchor::BottomLeft,
            font_colour: [1.0, 1.0, 1.0, 1.0],
            font_size: 30,
            ..Default::default()
        };

        let session_device_widget_template = SessionDeviceWidgetTemplate {
            dimensions: Dimensions { w: 300, h: 40 },
            device_id,
            device_name,
        };

        let session_devices = SessionDevicesWidget {
            position: PositionInit {
                x: 50,
                y: 400,
                z: 11,
            },
            session_device_widget_template,
        };

        let session_lobby_ui_expected = SessionLobbyUi {
            session_code,
            session_devices,
        };

        assert_eq!(session_lobby_ui_expected, session_lobby_ui);
    }
}
