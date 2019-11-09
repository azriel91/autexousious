#[cfg(test)]
mod test {
    use background_model::config::BackgroundDefinition;
    use indexmap::IndexMap;
    use sequence_model::config::{Sequence, SequenceEndTransition, Wait};
    use serde_yaml;
    use sprite_model::config::{SpriteFrame, SpritePosition, SpriteRef, SpriteSequence};

    use map_model::config::{MapBounds, MapDefinition, MapHeader};

    const MAP_NO_SPRITE_SEQUENCES: &str = r#"---
header:
  name: "Blank Map"
  bounds: { x: 1, y: 2, z: 3, width: 800, height: 600, depth: 200 }
"#;

    const MAP_WITH_SPRITE_SEQUENCES: &str = r#"---
header:
  name: "Map with sprite sequence"
  bounds: { x: 1, y: 2, z: 3, width: 800, height: 600, depth: 200 }

layers:
  zero:
    position: { x: 1, y: 4 } # missing z
    frames: [
      { wait: 7, sprite: { sheet: 0, index: 0 } },
      { wait: 7, sprite: { sheet: 0, index: 1 } },
    ]

  one:
    position: { x: -1, y: -2, z: -3 }
    frames: [{ wait: 1, sprite: { sheet: 0, index: 0 } }]
"#;

    #[test]
    fn deserialize_minimal_definition() {
        let map_definition = serde_yaml::from_str::<MapDefinition>(MAP_NO_SPRITE_SEQUENCES)
            .expect("Failed to deserialize map definition.");

        let bounds = MapBounds::new(1, 2, 3, 800, 600, 200);
        let header = MapHeader::new("Blank Map".to_string(), bounds);
        let expected = MapDefinition::new(header, BackgroundDefinition::default());

        assert_eq!(expected, map_definition);
    }

    #[test]
    fn deserialize_with_layers() {
        let map_definition = serde_yaml::from_str::<MapDefinition>(MAP_WITH_SPRITE_SEQUENCES)
            .expect("Failed to deserialize map definition.");

        let bounds = MapBounds::new(1, 2, 3, 800, 600, 200);
        let header = MapHeader::new("Map with sprite sequence".to_string(), bounds);
        let layer_0 = SpriteSequence::new(
            SpritePosition::new(1, 4, 0),
            Sequence::new(
                SequenceEndTransition::None,
                vec![
                    SpriteFrame {
                        wait: Wait::new(7),
                        sprite: SpriteRef::new(0, 0),
                        ..Default::default()
                    },
                    SpriteFrame {
                        wait: Wait::new(7),
                        sprite: SpriteRef::new(0, 1),
                        ..Default::default()
                    },
                ],
            ),
        );
        let layer_1 = SpriteSequence::new(
            SpritePosition::new(-1, -2, -3),
            Sequence::new(
                SequenceEndTransition::None,
                vec![SpriteFrame {
                    wait: Wait::new(1),
                    sprite: SpriteRef::new(0, 0),
                    ..Default::default()
                }],
            ),
        );
        let mut layers = IndexMap::new();
        layers.insert(String::from("zero"), layer_0);
        layers.insert(String::from("one"), layer_1);
        let expected = MapDefinition::new(header, BackgroundDefinition::new(layers));

        assert_eq!(expected, map_definition);
    }
}
