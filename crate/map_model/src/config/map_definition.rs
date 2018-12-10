use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{Layer, MapHeader};

/// Defines a playable area that objects can reside in.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, new)]
pub struct MapDefinition {
    /// Base information of the map.
    pub header: MapHeader,
    /// Image layers to draw.
    #[serde(default, rename = "layer")]
    pub layers: Vec<Layer>,
}

#[cfg(test)]
mod test {
    use sprite_model::config::SpriteFrame;
    use toml;

    use super::MapDefinition;
    use crate::config::{Layer, MapBounds, MapHeader, Position};

    const MAP_NO_LAYERS: &str = r#"
        [header]
        name   = "Blank Map"
        bounds = { x = 1, y = 2, z = 3, width = 800, height = 600, depth = 200 }
    "#;

    const MAP_WITH_LAYERS: &str = r#"
        [header]
        name   = "Layered Map"
        bounds = { x = 1, y = 2, z = 3, width = 800, height = 600, depth = 200 }

        [[layer]]
        position = { x = 1, y = 4 } # missing z
        frames = [
          { sheet = 0, sprite = 0, wait = 7 },
          { sheet = 0, sprite = 1, wait = 7 },
        ]

        [[layer]]
        position = { x = -1, y = -2, z = -3 }
        frames = [{ sheet = 0, sprite = 0, wait = 1 }]
    "#;

    #[test]
    fn deserialize_minimal_definition() {
        let map_definition = toml::from_str::<MapDefinition>(MAP_NO_LAYERS)
            .expect("Failed to deserialize map definition.");

        let bounds = MapBounds::new(1, 2, 3, 800, 600, 200);
        let header = MapHeader::new("Blank Map".to_string(), bounds);
        let expected = MapDefinition::new(header, Vec::new());
        assert_eq!(expected, map_definition);
    }

    #[test]
    fn deserialize_with_layers() {
        let map_definition = toml::from_str::<MapDefinition>(MAP_WITH_LAYERS)
            .expect("Failed to deserialize map definition.");

        let bounds = MapBounds::new(1, 2, 3, 800, 600, 200);
        let header = MapHeader::new("Layered Map".to_string(), bounds);
        let layer_0 = Layer::new(
            Position::new(1, 4, 0),
            vec![SpriteFrame::new(0, 0, 7), SpriteFrame::new(0, 1, 7)],
        );
        let layer_1 = Layer::new(Position::new(-1, -2, -3), vec![SpriteFrame::new(0, 0, 1)]);
        let layers = vec![layer_0, layer_1];
        let expected = MapDefinition::new(header, layers);
        assert_eq!(expected, map_definition);
    }
}
