#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use sequence_model::config::{Sequence, SequenceEndTransition, Wait};
    use serde_yaml;
    use sprite_model::config::{SpriteFrame, SpritePosition, SpriteRef, SpriteSequence};

    use background_model::config::BackgroundDefinition;

    const BACKGROUND_EMPTY: &str = "\
    ---\n\
    layers: {}\n
    ";
    const BACKGROUND_WITH_SPRITE_SEQUENCES: &str = r#"---
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
        let background_definition = serde_yaml::from_str::<BackgroundDefinition>(BACKGROUND_EMPTY)
            .expect("Failed to deserialize `BackgroundDefinition`.");

        let expected = BackgroundDefinition::default();

        assert_eq!(expected, background_definition);
    }

    #[test]
    fn deserialize_with_layers() {
        let background_definition =
            serde_yaml::from_str::<BackgroundDefinition>(BACKGROUND_WITH_SPRITE_SEQUENCES)
                .expect("Failed to deserialize `BackgroundDefinition`.");

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
        let expected = BackgroundDefinition::new(layers);

        assert_eq!(expected, background_definition);
    }
}
