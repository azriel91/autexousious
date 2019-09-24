use asset_derive::Asset;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{Layer, MapHeader};

/// Defines a playable area that objects can reside in.
#[derive(Asset, Clone, Debug, Deserialize, Serialize, PartialEq, new)]
pub struct MapDefinition {
    /// Base information of the map.
    pub header: MapHeader,
    /// Image layers to draw.
    #[serde(default, rename = "layer")]
    pub layers: Vec<Layer>,
}

#[cfg(test)]
mod test {
    use sequence_model::config::Wait;
    use serde_yaml;
    use sprite_model::config::SpriteRef;

    use super::MapDefinition;
    use crate::config::{Layer, LayerFrame, LayerPosition, MapBounds, MapHeader};

    const MAP_NO_LAYERS: &str = r#"---
header:
  name: "Blank Map"
  bounds: { x: 1, y: 2, z: 3, width: 800, height: 600, depth: 200 }
"#;

    const MAP_WITH_LAYERS: &str = r#"---
header:
  name: "Layered Map"
  bounds: { x: 1, y: 2, z: 3, width: 800, height: 600, depth: 200 }

layer:
  - position: { x: 1, y: 4 } # missing z
    frames: [
      { wait: 7, sprite: { sheet: 0, index: 0 } },
      { wait: 7, sprite: { sheet: 0, index: 1 } },
    ]

  - position: { x: -1, y: -2, z: -3 }
    frames: [{ wait: 1, sprite: { sheet: 0, index: 0 } }]
"#;

    #[test]
    fn deserialize_minimal_definition() {
        let map_definition = serde_yaml::from_str::<MapDefinition>(MAP_NO_LAYERS)
            .expect("Failed to deserialize map definition.");

        let bounds = MapBounds::new(1, 2, 3, 800, 600, 200);
        let header = MapHeader::new("Blank Map".to_string(), bounds);
        let expected = MapDefinition::new(header, Vec::new());
        assert_eq!(expected, map_definition);
    }

    #[test]
    fn deserialize_with_layers() {
        let map_definition = serde_yaml::from_str::<MapDefinition>(MAP_WITH_LAYERS)
            .expect("Failed to deserialize map definition.");

        let bounds = MapBounds::new(1, 2, 3, 800, 600, 200);
        let header = MapHeader::new("Layered Map".to_string(), bounds);
        let layer_0 = Layer::new(
            LayerPosition::new(1, 4, 0),
            vec![
                LayerFrame::new(Wait::new(7), SpriteRef::new(0, 0)),
                LayerFrame::new(Wait::new(7), SpriteRef::new(0, 1)),
            ],
        );
        let layer_1 = Layer::new(
            LayerPosition::new(-1, -2, -3),
            vec![LayerFrame::new(Wait::new(1), SpriteRef::new(0, 0))],
        );
        let layers = vec![layer_0, layer_1];
        let expected = MapDefinition::new(header, layers);
        assert_eq!(expected, map_definition);
    }
}
