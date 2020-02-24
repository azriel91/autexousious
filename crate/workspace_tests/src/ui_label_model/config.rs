#[cfg(test)]
mod test {
    use amethyst::ui::{Anchor, LineMode};
    use kinematic_model::config::PositionInit;
    use serde_yaml;
    use ui_model_spi::config::Dimensions;

    use ui_label_model::config::UiLabel;

    const UI_LABEL_YAML_ALL: &str = r#"
position: { x: 1, y: 2, z: 3 }
text: "Text"
dimensions: { w: 10, h: 20 }
align: "MiddleLeft"
line_mode: "Wrap"
font_colour: [0.1, 0.2, 0.3, 0.4]
font_size: 20
"#;

    const UI_LABEL_YAML_MIN: &str = "{}";

    #[test]
    fn deserialize_ui_label_all() {
        let ui_label = serde_yaml::from_str::<UiLabel>(UI_LABEL_YAML_ALL)
            .expect("Failed to deserialize `UiLabel`.");

        let ui_label_expected = UiLabel {
            position: PositionInit { x: 1, y: 2, z: 3 },
            text: String::from("Text"),
            dimensions: Dimensions { w: 10, h: 20 },
            align: Anchor::MiddleLeft,
            line_mode: LineMode::Wrap,
            font_colour: [0.1, 0.2, 0.3, 0.4],
            font_size: 20,
        };

        assert_eq!(ui_label_expected, ui_label);
    }

    #[test]
    fn deserialize_ui_label_min() {
        let ui_label = serde_yaml::from_str::<UiLabel>(UI_LABEL_YAML_MIN)
            .expect("Failed to deserialize `UiLabel`.");

        let ui_label_expected = UiLabel::default();

        assert_eq!(ui_label_expected, ui_label);
    }
}
