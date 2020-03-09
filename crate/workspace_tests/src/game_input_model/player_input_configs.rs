#[cfg(test)]
mod tests {
    use std::{collections::HashMap, convert::TryFrom};

    use amethyst::{
        input::{Axis as InputAxis, BindingError, Bindings, Button},
        winit::VirtualKeyCode,
    };
    use hamcrest::prelude::*;
    use indexmap::IndexMap;

    use game_input_model::config::{
        Axis, ControlAction, ControlBindings, ControllerConfig, PlayerActionControl,
        PlayerAxisControl, PlayerInputConfigs,
    };

    #[test]
    fn bindings_from_input_config_converts_correctly() {
        let controller_config_0 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::D, VirtualKeyCode::Key1]);
        let controller_config_1 = controller_config([
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::O,
        ]);

        let mut controller_configs = IndexMap::new();
        controller_configs.insert(String::from("zero1"), controller_config_0);
        controller_configs.insert(String::from("one"), controller_config_1);
        let player_input_configs = PlayerInputConfigs::new(controller_configs);

        let bindings = Bindings::<ControlBindings>::try_from(&player_input_configs)
            .expect("Failed to map `PlayerInputConfigs` into `Bindings`.");

        assert_that!(
            &bindings.axes().map(Clone::clone).collect::<Vec<_>>(),
            contains(vec![
                PlayerAxisControl::new(0, Axis::X),
                PlayerAxisControl::new(1, Axis::X)
            ])
        );
        // kcov-ignore-start
        assert_that!(
            // kcov-ignore-end
            &bindings.actions().map(Clone::clone).collect::<Vec<_>>(),
            contains(vec![
                PlayerActionControl::new(0, ControlAction::Jump),
                PlayerActionControl::new(1, ControlAction::Jump)
            ])
        );
    }

    #[test]
    fn try_from_returns_error_when_axis_key_bound_twice() {
        // Duplicate key A
        let controller_config_0 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::D, VirtualKeyCode::Key1]);
        let controller_config_1 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::Right, VirtualKeyCode::O]);

        let mut controller_configs = IndexMap::new();
        controller_configs.insert(String::from("zero1"), controller_config_0);
        controller_configs.insert(String::from("one"), controller_config_1);
        let player_input_configs = PlayerInputConfigs::new(controller_configs);

        if let Err(error) = Bindings::<ControlBindings>::try_from(&player_input_configs) {
            if let Some(binding_error) = error
                .source()
                .expect("Expected `BindingError` source.")
                .as_error()
                .downcast_ref::<Box<BindingError<ControlBindings>>>()
            {
                assert_eq!(
                    &Box::new(BindingError::AxisButtonAlreadyBoundToAxis(
                        PlayerAxisControl::new(0, Axis::X),
                        InputAxis::Emulated {
                            neg: Button::Key(VirtualKeyCode::A),
                            pos: Button::Key(VirtualKeyCode::D),
                        }
                    )),
                    binding_error
                );
            } else {
                // kcov-ignore-start
                panic!("Expected error type to be `Box<BindingError<ControlBindings>>`.");
                // kcov-ignore-end
            }
        } else {
            // kcov-ignore-start
            panic!("Expected to fail to map `PlayerInputConfigs` into `Bindings`.");
            // kcov-ignore-end
        }
    }

    #[test]
    fn try_from_returns_error_when_action_key_bound_twice() {
        // Duplicate key A
        let controller_config_0 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::D, VirtualKeyCode::Key1]);
        let controller_config_1 = controller_config([
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::Key1,
        ]);

        let mut controller_configs = IndexMap::new();
        controller_configs.insert(String::from("zero1"), controller_config_0);
        controller_configs.insert(String::from("one"), controller_config_1);
        let player_input_configs = PlayerInputConfigs::new(controller_configs);

        if let Err(error) = Bindings::<ControlBindings>::try_from(&player_input_configs) {
            if let Some(binding_error) = error
                .source()
                .expect("Expected `BindingError` source.")
                .as_error()
                .downcast_ref::<Box<BindingError<ControlBindings>>>()
            {
                let player = 0;
                let action = ControlAction::Jump;
                assert_eq!(
                    &Box::new(BindingError::ComboAlreadyBound(PlayerActionControl::new(
                        player, action
                    ))),
                    binding_error
                );
            } else {
                // kcov-ignore-start
                panic!("Expected error type to be `Box<BindingError<ControlBindings>>`.");
                // kcov-ignore-end
            }
        } else {
            // kcov-ignore-start
            panic!("Expected to fail to map `PlayerInputConfigs` into `Bindings`.");
            // kcov-ignore-end
        }
    }

    fn controller_config(keys: [VirtualKeyCode; 3]) -> ControllerConfig {
        let mut axes = HashMap::new();
        axes.insert(
            Axis::X,
            InputAxis::Emulated {
                neg: Button::Key(keys[0]),
                pos: Button::Key(keys[1]),
            },
        );
        let mut actions = HashMap::new();
        actions.insert(ControlAction::Jump, Button::Key(keys[2]));
        ControllerConfig::new(axes, actions)
    }
}
