#[cfg(test)]
mod test {
    use sequence_model::config::Wait;
    use serde_yaml;
    use sprite_model::config::{SpriteFrame, SpriteRef};

    use background_model::config::{BackgroundDefinition, Layer, LayerPosition};

    const BACKGROUND_EMPTY: &str = "\
    ---\n\
    layers: []\n
    ";
    const BACKGROUND_WITH_LAYERS: &str = r#"---
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
        let background_definition = serde_yaml::from_str::<BackgroundDefinition>(BACKGROUND_EMPTY)
            .expect("Failed to deserialize `BackgroundDefinition`.");

        let expected = BackgroundDefinition::new(Vec::new());

        assert_eq!(expected, background_definition);
    }

    #[test]
    fn deserialize_with_layers() {
        let background_definition =
            serde_yaml::from_str::<BackgroundDefinition>(BACKGROUND_WITH_LAYERS)
                .expect("Failed to deserialize `BackgroundDefinition`.");

        let layer_0 = Layer::new(
            LayerPosition::new(1, 4, 0),
            vec![
                SpriteFrame::new(Wait::new(7), SpriteRef::new(0, 0)),
                SpriteFrame::new(Wait::new(7), SpriteRef::new(0, 1)),
            ],
        );
        let layer_1 = Layer::new(
            LayerPosition::new(-1, -2, -3),
            vec![SpriteFrame::new(Wait::new(1), SpriteRef::new(0, 0))],
        );
        let layers = vec![layer_0, layer_1];
        let expected = BackgroundDefinition::new(layers);

        assert_eq!(expected, background_definition);
    }
}
