#[cfg(test)]
mod test {
    use kinematic_model::config::PositionInit;
    use serde_yaml;

    use asset_ui_model::config::{
        AssetDisplay, AssetDisplayGrid, AssetDisplayLayout, AssetSelector, Dimensions,
    };

    const ASSET_DISPLAY_YAML: &str = r#"---
position: { x: 100, y: 100, z: 12 }
layout:
  grid:
    column_count: 7
    cell_size: { w: 120, h: 120 }
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
        let asset_selector = serde_yaml::from_str::<AssetSelector<()>>(ASSET_DISPLAY_YAML)
            .expect("Failed to deserialize `AssetSelector`.");

        let position = PositionInit::new(100, 100, 12);
        let cell_size = Dimensions { w: 120, h: 120 };
        let asset_display_grid = AssetDisplayGrid {
            column_count: 7,
            cell_size,
        };
        let layout = AssetDisplayLayout::Grid(asset_display_grid);
        let asset_display = AssetDisplay::new(position, layout);
        let asset_selector_expected = AssetSelector { asset_display };
        assert_eq!(asset_selector_expected, asset_selector);
    }
}
