#[cfg(test)]
mod test {
    use std::str::FromStr;

    use amethyst::ui::{Anchor, LineMode};
    use application_menu::MenuIndex;
    use game_mode_selection_model::GameModeIndex;
    use indexmap::IndexMap;
    use input_reaction_model::config::{InputReaction, InputReactions};
    use kinematic_model::config::PositionInit;
    use sequence_model::config::{Sequence, SequenceEndTransition, SequenceNameString, Wait};
    use serde_yaml;
    use sprite_model::config::{SpriteFrame, SpriteRef};
    use ui_button_model::config::{UiButton, UiButtons};
    use ui_label_model::config::{UiLabel, UiSpriteLabel};
    use ui_menu_item_model::config::{UiMenuItem, UiMenuItems};
    use ui_model_spi::config::{Dimensions, WidgetStatus, WidgetStatusSequences};

    use ui_model::config::{UiDefinition, UiFrame, UiSequence, UiSequences, UiType};

    const UI_MENU_YAML: &str = r#"
menu:
  # First item is active by default. The sequence here should correspond to the active status.
  - index: "start_game"
    label: { text: "Start Game", dimensions: { w: 200, h: 50 }, font_colour: [0.1, 0.1, 0.1, 0.1], font_size: 20 }
    position: { x: -1, y: -2, z: -3 }
    sprite: { sequence: "active" }
    widget_status_sequences:
      idle: "start_game_inactive"
      active: "active"

  - index: "exit"
    label: { position: { x: 1, y: 2, z: 3 }, text: "Exit" }
    position: { x: -1, y: -2, z: -3 }
    sprite: { sequence: "exit_inactive" }
    widget_status_sequences:
      idle: "start_game_inactive"
      active: "active"

buttons:
  - position: { x: -4, y: -5, z: -6 }
    label: { position: { x: -7, y: -8, z: -9 }, text: "Button Zero" }
    sprite: { position: { x: -10, y: -11, z: -12 }, sequence: "button_inactive" }

display_control_buttons: true

sequences:
  start_game_inactive:
    next: "none"
    input_reactions:
      press_defend: "button_inactive"
    frames:
      - wait: 2
        sprite: { sheet: 0, index: 0 }
        input_reactions:
          press_attack: "active"

  active:
    next: "repeat"
    input_reactions:
      press_defend: "button_inactive"
    frames:
      - wait: 2
        sprite: { sheet: 0, index: 0 }
        input_reactions:
          press_attack: "active"

  exit_inactive:
    next: "none"
    input_reactions:
      press_defend: "button_inactive"
    frames:
      - wait: 2
        sprite: { sheet: 0, index: 0 }
        input_reactions:
          press_attack: "active"

  button_inactive:
    next: "none"
    input_reactions:
      press_defend: "button_inactive"
    frames:
      - wait: 2
        sprite: { sheet: 0, index: 0 }
        input_reactions:
          press_attack: "active"
"#;

    #[test]
    fn deserialize_ui_definition() {
        let ui_definition = serde_yaml::from_str::<UiDefinition>(UI_MENU_YAML)
            .expect("Failed to deserialize `UiDefinition`.");

        let position_init = PositionInit::new(-1, -2, -3);
        let widget_status_sequences = {
            let mut widget_status_sequences = IndexMap::new();
            widget_status_sequences.insert(
                WidgetStatus::Idle,
                SequenceNameString::String(String::from("start_game_inactive")),
            );
            widget_status_sequences.insert(
                WidgetStatus::Active,
                SequenceNameString::String(String::from("active")),
            );
            WidgetStatusSequences::new(widget_status_sequences)
        };
        let ui_type = UiType::Menu(UiMenuItems::new(vec![
            UiMenuItem::new(
                position_init,
                UiLabel {
                    position: PositionInit::new(0, 0, 0),
                    text: String::from("Start Game"),
                    dimensions: Dimensions { w: 200, h: 50 },
                    align: Anchor::Middle,
                    line_mode: LineMode::Single,
                    font_colour: [0.1; 4],
                    font_size: 20,
                },
                UiSpriteLabel::new(
                    PositionInit::new(0, 0, 0),
                    SequenceNameString::String(String::from("active")),
                ),
                MenuIndex::GameMode(GameModeIndex::StartGame),
                widget_status_sequences.clone(),
            ),
            UiMenuItem::new(
                position_init,
                UiLabel {
                    position: PositionInit::new(1, 2, 3),
                    text: String::from("Exit"),
                    ..Default::default()
                },
                UiSpriteLabel::new(
                    PositionInit::new(0, 0, 0),
                    SequenceNameString::from_str("exit_inactive").expect(
                        "Expected `SequenceNameString::from_str(\"exit_inactive\")` to succeed.",
                    ),
                ),
                MenuIndex::GameMode(GameModeIndex::Exit),
                widget_status_sequences,
            ),
        ]));
        let buttons = UiButtons::new(vec![UiButton::new(
            PositionInit::new(-4, -5, -6),
            UiLabel {
                position: PositionInit::new(-7, -8, -9),
                text: String::from("Button Zero"),
                ..Default::default()
            },
            UiSpriteLabel::new(
                PositionInit::new(-10, -11, -12),
                SequenceNameString::String(String::from("button_inactive")),
            ),
        )]);
        let mut input_reactions = InputReactions::default();
        input_reactions.press_defend = Some(InputReaction::SequenceNameString(
            SequenceNameString::String(String::from("button_inactive")),
        ));
        let sequences = {
            let mut sequences = IndexMap::new();
            sequences.insert(
                SequenceNameString::String(String::from("start_game_inactive")),
                UiSequence::new(
                    Sequence::new(SequenceEndTransition::None, ui_frames()),
                    Some(input_reactions.clone()),
                ),
            );
            sequences.insert(
                SequenceNameString::String(String::from("active")),
                UiSequence::new(
                    Sequence::new(SequenceEndTransition::Repeat, ui_frames()),
                    Some(input_reactions.clone()),
                ),
            );
            sequences.insert(
                SequenceNameString::String(String::from("exit_inactive")),
                UiSequence::new(
                    Sequence::new(SequenceEndTransition::None, ui_frames()),
                    Some(input_reactions.clone()),
                ),
            );
            sequences.insert(
                SequenceNameString::String(String::from("button_inactive")),
                UiSequence::new(
                    Sequence::new(SequenceEndTransition::None, ui_frames()),
                    Some(input_reactions),
                ),
            );
            UiSequences::new(sequences)
        };
        let ui_definition_expected = UiDefinition {
            ui_type,
            buttons,
            display_control_buttons: true,
            sequences,
        };

        assert_eq!(ui_definition_expected, ui_definition);
    }

    fn ui_frames() -> Vec<UiFrame> {
        let mut input_reactions = InputReactions::default();
        input_reactions.press_attack = Some(InputReaction::SequenceNameString(
            SequenceNameString::String(String::from("active")),
        ));

        vec![UiFrame {
            sprite_frame: SpriteFrame {
                wait: Wait::new(2),
                sprite: SpriteRef::new(0, 0),
                ..Default::default()
            },
            input_reactions,
        }]
    }
}
