pub use self::sprite_offset::SpriteOffset;
pub use self::sprite_sheet_definition::SpriteSheetDefinition;
pub use self::sprites_definition::SpritesDefinition;

mod sprite_offset;
mod sprite_sheet_definition;
mod sprites_definition;

#[cfg(test)]
mod test {
    use toml;

    use super::SpritesDefinition;

    const SPRITES_TOML: &str = r#"
        [[sheets]]
        # 0
        path         = "heat_defense.png"
        sprite_w     = 79.0
        sprite_h     = 79.0
        row_count    = 1
        column_count = 2
        offsets = [
          { x =  35, y =  79 }, # 0
          { x = 115, y =  79 }, # 1
        ]

        [[sheets]]
        # 1
        path         = "heat_defense.png"
        sprite_w     = 79.0
        sprite_h     = 79.0
        row_count    = 2
        column_count = 3
        offsets = [
          { x =  35, y =  79 }, # 0
          { x = 115, y =  79 }, # 1
          { x = 195, y =  79 }, # 2
          { x =  35, y = 159 }, # 3
          { x = 115, y = 159 }, # 4
          { x = 195, y = 159 }, # 5
        ]
        "#;

    #[test]
    fn deserialize_multiple_sheets() {
        let sprites_definition = toml::from_str::<SpritesDefinition>(SPRITES_TOML)
            .expect("Failed to deserialize sprites definition.");

        assert_eq!(2, sprites_definition.sheets.len());
    }

    #[test]
    fn has_border_defaults_to_true() {
        let sprites_definition = toml::from_str::<SpritesDefinition>(SPRITES_TOML)
            .expect("Failed to deserialize sprites definition.");

        assert!(sprites_definition.sheets[1].has_border);
    }

    #[test]
    fn allows_negative_sprite_offsets() {
        let sprites_toml = r#"
            [[sheets]]
            # 0
            path         = "heat_defense.png"
            sprite_w     = 79.0
            sprite_h     = 79.0
            row_count    = 1
            column_count = 2
            offsets = [
              { x = -35, y =  79 }, # 0
              { x = 115, y = -79 }, # 1
            ]
            "#;
        let sprites_definition = toml::from_str::<SpritesDefinition>(sprites_toml)
            .expect("Failed to deserialize sprites definition.");

        assert_eq!(-35, sprites_definition.sheets[0].offsets[0].x);
        assert_eq!(-79, sprites_definition.sheets[0].offsets[1].y);
    }
}
