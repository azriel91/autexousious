use config::MapHeader;

/// Defines a playable area that objects can reside in.
#[derive(Clone, Debug, Deserialize, PartialEq, new)]
pub struct MapDefinition {
    /// Base information of the map.
    pub header: MapHeader,
}

#[cfg(test)]
mod test {
    use toml;

    use super::MapDefinition;
    use config::{MapBounds, MapHeader};

    const MAP_TOML: &str = r#"
        [header]
        name = "Blank Map"
        bounds = { x = 1, y = 2, z = 3, width = 800, height = 600, depth = 200 }
    "#;

    #[test]
    fn deserialize_map_definition() {
        let map_definition = toml::from_str::<MapDefinition>(MAP_TOML)
            .expect("Failed to deserialize map definition.");

        let bounds = MapBounds::new(1, 2, 3, 800, 600, 200);
        let header = MapHeader::new("Blank Map".to_string(), bounds);
        let expected = MapDefinition::new(header);
        assert_eq!(expected, map_definition);
    }
}
