#[cfg(test)]
mod test {
    use background_model::config::BackgroundDefinition;
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
                    SpriteFrame::new(Wait::new(7), SpriteRef::new(0, 0)),
                    SpriteFrame::new(Wait::new(7), SpriteRef::new(0, 1)),
                ],
            ),
        );
        let layer_1 = SpriteSequence::new(
            SpritePosition::new(-1, -2, -3),
            Sequence::new(
                SequenceEndTransition::None,
                vec![SpriteFrame::new(Wait::new(1), SpriteRef::new(0, 0))],
            ),
        );
        let layers = vec![layer_0, layer_1];
        let expected = MapDefinition::new(header, BackgroundDefinition::new(layers));

        assert_eq!(expected, map_definition);
    }
}
