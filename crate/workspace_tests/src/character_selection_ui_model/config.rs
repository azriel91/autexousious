#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use kinematic_model::config::PositionInit;
    use sequence_model::config::SequenceNameString;
    use serde_yaml;
    use ui_label_model::config::UiSpriteLabel;

    use character_selection_ui_model::config::{
        CharacterSelectionUi, CswDefinition, CswPortraits, CswTemplate,
    };

    const CHARACTER_SELECTION_UI_YAML: &str = r#"---
widgets:
  - position: { x: 100, y: 300 } # p0
  - position: { x: 400, y: 300 } # p1
  - position: { x: 100, y: 100 } # p2
  - position: { x: 400, y: 100 } # p3

widget_template:
  portraits:
    join: "portrait_press_to_join"
    random: "portrait_random"

  layers:
    outer_frame:
      sequence: "widget_inactive"
      position: { x: 0, y: 0 }
    portrait:
      sequence: "portrait_press_to_join"
      position: { x: 0, y: 0 }
"#;

    #[test]
    fn deserialize_character_selection_ui() {
        let character_selection_ui =
            serde_yaml::from_str::<CharacterSelectionUi>(CHARACTER_SELECTION_UI_YAML)
                .expect("Failed to deserialize `CharacterSelectionUi`.");

        let widgets = vec![
            CswDefinition {
                position: PositionInit::new(100, 300, 0),
            },
            CswDefinition {
                position: PositionInit::new(400, 300, 0),
            },
            CswDefinition {
                position: PositionInit::new(100, 100, 0),
            },
            CswDefinition {
                position: PositionInit::new(400, 100, 0),
            },
        ];
        let portraits = CswPortraits {
            join: SequenceNameString::String(String::from("portrait_press_to_join")),
            random: SequenceNameString::String(String::from("portrait_random")),
        };
        let outer_frame_label = UiSpriteLabel {
            sequence: SequenceNameString::String(String::from("widget_inactive")),
            position: PositionInit::new(0, 0, 0),
        };
        let portrait_label = UiSpriteLabel {
            sequence: SequenceNameString::String(String::from("portrait_press_to_join")),
            position: PositionInit::new(0, 0, 0),
        };
        let mut layers = IndexMap::new();
        layers.insert(String::from("outer_frame"), outer_frame_label);
        layers.insert(String::from("portrait"), portrait_label);
        let widget_template = CswTemplate { portraits, layers };
        let character_selection_ui_expected = CharacterSelectionUi {
            widgets,
            widget_template,
        };
        assert_eq!(character_selection_ui_expected, character_selection_ui);
    }
}
