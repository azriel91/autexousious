#[cfg(test)]
mod test {
    use std::str::FromStr;

    use game_mode_selection_model::GameModeIndex;
    use sequence_model::config::SequenceNameString;
    use serde_yaml;
    use ui_menu_item_model::config::{UiMenuItem, UiMenuItemSequenceName, UiMenuItems};

    use ui_model::config::UiType;

    const UI_MENU_YAML: &str = r#"
menu:
  # First item is active by default. The sequence here should correspond to the active status.
  - index: "start_game"
    text: "Start Game"
    sequence: "active"

  - index: "exit"
    text: "Exit"
    sequence: "exit_inactive"
"#;

    #[test]
    fn deserialize_ui_type() {
        let ui_type = serde_yaml::from_str::<UiType<_>>(UI_MENU_YAML)
            .expect("Failed to deserialize `UiType`.");

        assert_eq!(
            UiType::Menu(UiMenuItems::new(vec![
                UiMenuItem::new(
                    GameModeIndex::StartGame,
                    String::from("Start Game"),
                    SequenceNameString::from(UiMenuItemSequenceName::Active),
                ),
                UiMenuItem::new(
                    GameModeIndex::Exit,
                    String::from("Exit"),
                    SequenceNameString::from_str("exit_inactive").expect(
                        "Expected `SequenceNameString::from_str(\"exit_inactive\")` to succeed."
                    ),
                )
            ])),
            ui_type
        );
    }
}
