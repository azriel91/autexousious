#[cfg(test)]
mod test {
    use serde_yaml;

    use sequence_model::config::Wait;
    use sprite_model::config::{Scale, SpriteFrame, SpriteRef, Tint};

    const SPRITE_FRAME_DEFAULTS_YAML: &str = "{}";
    const SPRITE_FRAME_FULL_YAML: &str = r#"---
wait: 3
sprite: { sheet: 1, index: 2 }
tint: { r: 0.1, g: 0.2, b: 0.3, a: 0.4 }
scale: 2.0
"#;

    #[test]
    fn deserialize_sprite_frame_defaults() {
        let sprite_frame = serde_yaml::from_str::<SpriteFrame>(SPRITE_FRAME_DEFAULTS_YAML)
            .expect("Failed to deserialize `SpriteFrame` defaults.");

        assert_eq!(
            SpriteFrame {
                wait: Wait::new(2),
                sprite: SpriteRef::new(0, 0),
                tint: Tint::new(1., 1., 1., 1.),
                scale: Scale::new(1.),
            },
            sprite_frame
        );
    }

    #[test]
    fn deserialize_sprite_frame_full() {
        let sprite_frame = serde_yaml::from_str::<SpriteFrame>(SPRITE_FRAME_FULL_YAML)
            .expect("Failed to deserialize `SpriteFrame` full.");

        let sprite_frame_expected = SpriteFrame {
            wait: Wait::new(3),
            sprite: SpriteRef::new(1, 2),
            tint: Tint::new(0.1, 0.2, 0.3, 0.4),
            scale: Scale::new(2.),
        };
        assert_eq!(sprite_frame_expected, sprite_frame);
    }
}
