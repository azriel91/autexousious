#[cfg(test)]
mod test {
    use std::str::FromStr;

    use game_mode_selection_model::GameModeIndex;
    use indexmap::IndexMap;
    use sequence_model::config::{Sequence, SequenceEndTransition, SequenceNameString, Wait};
    use serde_yaml;
    use sprite_model::config::{SpriteFrame, SpritePosition, SpriteRef};
    use ui_menu_item_model::config::{UiMenuItem, UiMenuItems};
    use ui_model_spi::config::UiSequenceName;

    use ui_model::config::{UiDefinition, UiSequence, UiSequences, UiType};

    const UI_MENU_YAML: &str = r#"
menu:
  # First item is active by default. The sequence here should correspond to the active status.
  - index: "start_game"
    text: "Start Game"
    sequence: "active"

  - index: "exit"
    text: "Exit"
    sequence: "exit_inactive"

sequences:
  start_game_inactive:
    next: "none"
    position: { x: -1, y: -2, z: -3 }
    frames:
      - { wait: 2, sprite: { sheet: 0, index: 0 } }

  active:
    next: "repeat"
    position: { x: -1, y: -2, z: -3 }
    frames:
      - { wait: 2, sprite: { sheet: 0, index: 0 } }

  exit_inactive:
    next: "none"
    position: { x: -1, y: -2, z: -3 }
    frames:
      - { wait: 2, sprite: { sheet: 0, index: 0 } }
"#;

    #[test]
    fn deserialize_ui_definition() {
        let ui_definition = serde_yaml::from_str::<UiDefinition>(UI_MENU_YAML)
            .expect("Failed to deserialize `UiDefinition`.");

        let ui_type = UiType::Menu(UiMenuItems::new(vec![
            UiMenuItem::new(
                GameModeIndex::StartGame,
                String::from("Start Game"),
                SequenceNameString::from(UiSequenceName::Active),
            ),
            UiMenuItem::new(
                GameModeIndex::Exit,
                String::from("Exit"),
                SequenceNameString::from_str("exit_inactive").expect(
                    "Expected `SequenceNameString::from_str(\"exit_inactive\")` to succeed.",
                ),
            ),
        ]));
        let sprite_position = SpritePosition::new(-1, -2, -3);
        let sequences = {
            let mut sequences = IndexMap::new();
            sequences.insert(
                SequenceNameString::String(String::from("start_game_inactive")),
                UiSequence::new(
                    sprite_position,
                    Sequence::new(SequenceEndTransition::None, sprite_frames()),
                ),
            );
            sequences.insert(
                SequenceNameString::Name(UiSequenceName::Active),
                UiSequence::new(
                    sprite_position,
                    Sequence::new(SequenceEndTransition::Repeat, sprite_frames()),
                ),
            );
            sequences.insert(
                SequenceNameString::String(String::from("exit_inactive")),
                UiSequence::new(
                    sprite_position,
                    Sequence::new(SequenceEndTransition::None, sprite_frames()),
                ),
            );
            UiSequences::new(sequences)
        };
        let ui_definition_expected = UiDefinition { ui_type, sequences };

        assert_eq!(ui_definition_expected, ui_definition);
    }

    fn sprite_frames() -> Vec<SpriteFrame> {
        vec![SpriteFrame {
            wait: Wait::new(2),
            sprite: SpriteRef::new(0, 0),
            ..Default::default()
        }]
    }
}
