use config::Layer;
use config::MapHeader;

/// Defines a playable area that objects can reside in.
#[derive(Clone, Debug, Deserialize, PartialEq, new)]
pub struct MapDefinition {
    /// Base information of the map.
    pub header: MapHeader,
    /// Image layers to draw.
    #[serde(default, rename = "layer")]
    pub layers: Vec<Layer>,
}

#[cfg(test)]
mod test {
    use toml;

    use super::MapDefinition;
    use config::{Layer, MapBounds, MapHeader, Position};

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
        path     = "image_0.png"
        width    = 800
        height   = 198
        position = { x = 1, y = 4 } # missing z

        [[layer]]
        path     = "image_1.png"
        width    = 50
        height   = 40
        position = { x = -1, y = -2, z = -3 }
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
        let layer_0 = Layer::new("image_0.png".to_string(), 800, 198, Position::new(1, 4, 0));
        let layer_1 = Layer::new("image_1.png".to_string(), 50, 40, Position::new(-1, -2, -3));
        let layers = vec![layer_0, layer_1];
        let expected = MapDefinition::new(header, layers);
        assert_eq!(expected, map_definition);
    }
}
