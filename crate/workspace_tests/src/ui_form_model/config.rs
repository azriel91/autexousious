#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use kinematic_model::config::PositionInit;
    use sequence_model::config::SequenceNameString;
    use serde_yaml;
    use ui_form_model::config::UiTextInput;
    use ui_label_model::config::{UiLabel, UiSpriteLabel};
    use ui_model_spi::config::{Dimensions, WidgetStatus, WidgetStatusSequences};

    use ui_form_model::config::UiFormItem;

    const UI_FORM_ITEM_YAML_ALL: &str = r#"
position: { x: 1, y: 2, z: 3 }
label:
  position: { x: 4, y: 5, z: 6 }
  text: "Host a game:"
sprite:
  position: { x: 7, y: 8, z: 9 }
  sequence: "active"
widget_status_sequences:
  idle: "host_game_inactive"
  active: "active"
input_field:
  position: { x: 1, y: 2, z: 3 }
  text: "Text"
  dimensions: { w: 10, h: 20 }
  font_colour: [0.1, 0.2, 0.3, 0.4]
  font_size: 20
  max_length: 99
  selected_text_colour: [0.1, 0.2, 0.3, 0.4]
  selected_background_colour: [0.5, 0.6, 0.7, 0.8]
"#;

    const UI_FORM_ITEM_YAML_MIN: &str = r#"
sprite: { sequence: "active" }
"#;

    #[test]
    fn deserialize_ui_form_item_all() {
        let ui_form_item = serde_yaml::from_str::<UiFormItem>(UI_FORM_ITEM_YAML_ALL)
            .expect("Failed to deserialize `UiFormItem`.");

        let position = PositionInit { x: 1, y: 2, z: 3 };
        let label = UiLabel {
            position: PositionInit { x: 4, y: 5, z: 6 },
            text: String::from("Host a game:"),
            ..Default::default()
        };
        let sprite = UiSpriteLabel::new(
            PositionInit { x: 7, y: 8, z: 9 },
            SequenceNameString::String(String::from("active")),
        );
        let widget_status_sequences = {
            let mut widget_status_sequences = IndexMap::new();
            widget_status_sequences.insert(
                WidgetStatus::Idle,
                SequenceNameString::String(String::from("host_game_inactive")),
            );
            widget_status_sequences.insert(
                WidgetStatus::Active,
                SequenceNameString::String(String::from("active")),
            );
            WidgetStatusSequences::new(widget_status_sequences)
        };
        let input_field = UiTextInput {
            label_attributes: UiLabel {
                position: PositionInit { x: 1, y: 2, z: 3 },
                text: String::from("Text"),
                dimensions: Dimensions { w: 10, h: 20 },
                font_colour: [0.1, 0.2, 0.3, 0.4],
                font_size: 20,
            },
            max_length: 99,
            selected_text_colour: [0.1, 0.2, 0.3, 0.4],
            selected_background_colour: [0.5, 0.6, 0.7, 0.8],
        };
        let ui_form_item_expected = UiFormItem {
            position,
            label,
            sprite,
            widget_status_sequences,
            input_field,
        };

        assert_eq!(ui_form_item_expected, ui_form_item);
    }

    #[test]
    fn deserialize_ui_form_item_min() {
        let ui_form_item = serde_yaml::from_str::<UiFormItem>(UI_FORM_ITEM_YAML_MIN)
            .expect("Failed to deserialize `UiFormItem`.");

        let sprite = UiSpriteLabel::new(
            PositionInit::default(),
            SequenceNameString::String(String::from("active")),
        );
        let ui_form_item_expected = UiFormItem {
            position: Default::default(),
            label: Default::default(),
            sprite,
            widget_status_sequences: Default::default(),
            input_field: Default::default(),
        };

        assert_eq!(ui_form_item_expected, ui_form_item);
    }
}
