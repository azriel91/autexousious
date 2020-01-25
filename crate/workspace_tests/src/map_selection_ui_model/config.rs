#[cfg(test)]
mod test {
    use asset_ui_model::config::{
        AshTemplate, AssetDisplay, AssetDisplayGrid, AssetDisplayLayout, AssetSelector, Dimensions,
    };
    use indexmap::IndexMap;
    use kinematic_model::config::PositionInit;
    use sequence_model::config::SequenceNameString;
    use serde_yaml;
    use ui_label_model::config::UiSpriteLabel;

    use map_selection_ui_model::config::{
        MapSelectionUi, MpwTemplate, MswLayer, MswLayerName, MswPortraits,
    };

    const MAP_SELECTION_UI_YAML: &str = r#"---
map_preview:
  position: { x: 100, y: 300 }
  dimensions: { w: 400, h: 300 }
  portraits:
    random: "portrait_random"
    select: "portrait_select"

  layers:
    main:
      sequence: "widget_inactive"
      position: { x: 0, y: 0 }
    portrait:
      sequence: "widget_portrait"
      position: { x: 0, y: 0 }
    other_layer:
      sequence: "other"
      position: { x: 0, y: 0 }

maps_available_selector:
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
    fn deserialize_map_selection_ui() {
        let map_selection_ui = serde_yaml::from_str::<MapSelectionUi>(MAP_SELECTION_UI_YAML)
            .expect("Failed to deserialize `MapSelectionUi`.");

        let portraits = MswPortraits {
            random: SequenceNameString::String(String::from("portrait_random")),
            select: SequenceNameString::String(String::from("portrait_select")),
        };
        let main_label = UiSpriteLabel {
            sequence: SequenceNameString::String(String::from("widget_inactive")),
            position: PositionInit::new(0, 0, 0),
        };
        let portrait_label = UiSpriteLabel {
            sequence: SequenceNameString::String(String::from("widget_portrait")),
            position: PositionInit::new(0, 0, 0),
        };
        let other_label = UiSpriteLabel {
            sequence: SequenceNameString::String(String::from("other")),
            position: PositionInit::new(0, 0, 0),
        };
        let mut layers = IndexMap::new();
        layers.insert(MswLayer::Name(MswLayerName::Main), main_label);
        layers.insert(MswLayer::Name(MswLayerName::Portrait), portrait_label);
        layers.insert(MswLayer::String(String::from("other_layer")), other_label);
        let position = PositionInit::new(100, 300, 0);
        let dimensions = Dimensions { w: 400, h: 300 };
        let map_preview = MpwTemplate {
            position,
            dimensions,
            portraits,
            layers,
        };

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
        let maps_available_selector = AssetSelector {
            asset_display,
            selection_highlights,
        };
        let map_selection_ui_expected = MapSelectionUi {
            map_preview,
            maps_available_selector,
        };
        assert_eq!(map_selection_ui_expected, map_selection_ui);
    }
}
