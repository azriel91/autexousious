mod sprite_sheet_definition;

#[cfg(test)]
mod test {
    use serde_yaml;

    use sprite_model::config::SpritesDefinition;

    const SPRITES_YAML: &str = r#"---
sheets:
  - # 0
    path        : "heat_defense.png"
    sprite_w    : 79
    sprite_h    : 79
    row_count   : 1
    column_count: 2
    offsets:
      - { x:  35, y:  79 } # 0
      - { x: 115, y:  79 } # 1

  - # 1
    path        : "heat_defense.png"
    sprite_w    : 79
    sprite_h    : 79
    row_count   : 2
    column_count: 3
    offsets:
      - { x:  35, y:  79 } # 0
      - { x: 115, y:  79 } # 1
      - { x: 195, y:  79 } # 2
      - { x:  35, y: 159 } # 3
      - { x: 115, y: 159 } # 4
      - { x: 195, y: 159 } # 5
"#;

    #[test]
    fn deserialize_multiple_sheets() {
        let sprites_definition = serde_yaml::from_str::<SpritesDefinition>(SPRITES_YAML)
            .expect("Failed to deserialize sprites definition.");

        assert_eq!(2, sprites_definition.sheets.len());
    }

    #[test]
    fn has_border_defaults_to_true() {
        let sprites_definition = serde_yaml::from_str::<SpritesDefinition>(SPRITES_YAML)
            .expect("Failed to deserialize sprites definition.");

        assert!(sprites_definition.sheets[1].has_border);
    }

    #[test]
    fn allows_negative_sprite_offsets() {
        let sprites_yaml = r#"---
sheets:
  - # 0
    path        : "heat_defense.png"
    sprite_w    : 79
    sprite_h    : 79
    row_count   : 1
    column_count: 2
    offsets:
      - { x: -35, y:  79 } # 0
      - { x: 115, y: -79 } # 1
"#;
        let sprites_definition = serde_yaml::from_str::<SpritesDefinition>(sprites_yaml)
            .expect("Failed to deserialize sprites definition.");

        let offsets = sprites_definition.sheets[0].offsets.as_ref().unwrap();
        assert_eq!(-35, offsets[0].x);
        assert_eq!(-79, offsets[1].y);
    }
}
