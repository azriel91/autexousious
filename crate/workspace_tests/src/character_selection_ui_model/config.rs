#[cfg(test)]
mod test {
    use asset_ui_model::config::{
        AshTemplate, AssetDisplay, AssetDisplayGrid, AssetDisplayLayout, AssetSelector,
        AswPortraitName, AswPortraits, Dimensions,
    };
    use indexmap::IndexMap;
    use kinematic_model::config::PositionInit;
    use sequence_model::config::SequenceNameString;
    use serde_yaml;
    use ui_label_model::config::UiSpriteLabel;

    use character_selection_ui_model::config::{
        CharacterSelectionUi, CswDefinition, CswLayer, CswLayerName, CswTemplate,
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
    select: "portrait_select"

  layers:
    main:
      sequence: "widget_inactive"
      position: { x: 0, y: 0 }
    portrait:
      sequence: "portrait_press_to_join"
      position: { x: 0, y: 0 }
    other_layer:
      sequence: "other"
      position: { x: 0, y: 0 }

characters_available_selector:
  position: { x: 100, y: 100, z: 12 }
  layout:
    grid:
      column_count: 7
      cell_size: { w: 120, h: 120 }

  selection_highlights:
    - position: { x: 0, y: -15, z: 0 }
      sequence: "p0_highlight"
    - position: { x: 20, y: -15, z: 0 }
      sequence: "p1_highlight"
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
        let portraits = {
            let mut asw_portraits = AswPortraits::default();
            asw_portraits.insert(
                AswPortraitName::Join,
                SequenceNameString::String(String::from("portrait_press_to_join")),
            );
            asw_portraits.insert(
                AswPortraitName::Random,
                SequenceNameString::String(String::from("portrait_random")),
            );
            asw_portraits.insert(
                AswPortraitName::Select,
                SequenceNameString::String(String::from("portrait_select")),
            );
            asw_portraits
        };
        let main_label = UiSpriteLabel {
            sequence: SequenceNameString::String(String::from("widget_inactive")),
            position: PositionInit::new(0, 0, 0),
        };
        let portrait_label = UiSpriteLabel {
            sequence: SequenceNameString::String(String::from("portrait_press_to_join")),
            position: PositionInit::new(0, 0, 0),
        };
        let other_label = UiSpriteLabel {
            sequence: SequenceNameString::String(String::from("other")),
            position: PositionInit::new(0, 0, 0),
        };
        let mut layers = IndexMap::new();
        layers.insert(CswLayer::Name(CswLayerName::Main), main_label);
        layers.insert(CswLayer::Name(CswLayerName::Portrait), portrait_label);
        layers.insert(CswLayer::String(String::from("other_layer")), other_label);
        let widget_template = CswTemplate { portraits, layers };
        let position = PositionInit::new(100, 100, 12);
        let cell_size = Dimensions { w: 120, h: 120 };
        let asset_display_grid = AssetDisplayGrid {
            column_count: 7,
            cell_size,
        };
        let layout = AssetDisplayLayout::Grid(asset_display_grid);
        let asset_display = AssetDisplay::new(position, layout);
        let selection_0 = AshTemplate::new(UiSpriteLabel {
            position: PositionInit::new(0, -15, 0),
            sequence: SequenceNameString::String(String::from("p0_highlight")),
        });
        let selection_1 = AshTemplate::new(UiSpriteLabel {
            position: PositionInit::new(20, -15, 0),
            sequence: SequenceNameString::String(String::from("p1_highlight")),
        });
        let selection_highlights = vec![selection_0, selection_1];
        let characters_available_selector = AssetSelector {
            asset_display,
            selection_highlights,
        };
        let character_selection_ui_expected = CharacterSelectionUi {
            widgets,
            widget_template,
            characters_available_selector,
        };
        assert_eq!(character_selection_ui_expected, character_selection_ui);
    }
}
