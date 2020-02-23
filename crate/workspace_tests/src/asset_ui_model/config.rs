#[cfg(test)]
mod test {
    use kinematic_model::config::PositionInit;
    use sequence_model::config::SequenceNameString;
    use serde_yaml;
    use ui_label_model::config::UiSpriteLabel;
    use ui_model_spi::config::Dimensions;

    use asset_ui_model::config::{
        AshTemplate, AssetDisplay, AssetDisplayGrid, AssetDisplayLayout, AssetSelector,
    };

    const ASSET_DISPLAY_YAML: &str = r#"---
position: { x: 100, y: 100, z: 12 }
layout:
  grid:
    column_count: 7
    cell_size: { w: 120, h: 120 }
"#;

    const ASSET_SELECTOR_YAML: &str = r#"---
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
    fn deserialize_asset_display() {
        let asset_display = serde_yaml::from_str::<AssetDisplay<()>>(ASSET_DISPLAY_YAML)
            .expect("Failed to deserialize `AssetDisplay`.");

        let position = PositionInit::new(100, 100, 12);
        let cell_size = Dimensions { w: 120, h: 120 };
        let asset_display_grid = AssetDisplayGrid {
            column_count: 7,
            cell_size,
        };
        let layout = AssetDisplayLayout::Grid(asset_display_grid);
        let asset_display_expected = AssetDisplay::new(position, layout);
        assert_eq!(asset_display_expected, asset_display);
    }

    #[test]
    fn deserialize_asset_selector() {
        let asset_selector = serde_yaml::from_str::<AssetSelector<()>>(ASSET_SELECTOR_YAML)
            .expect("Failed to deserialize `AssetSelector`.");

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
        let asset_selector_expected = AssetSelector {
            asset_display,
            selection_highlights,
        };
        assert_eq!(asset_selector_expected, asset_selector);
    }
}
