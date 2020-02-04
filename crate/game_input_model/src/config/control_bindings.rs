use amethyst::input::BindingTypes;

use crate::config::{PlayerActionControl, PlayerAxisControl};

/// Type used for Amethyst input bindings.
#[derive(Debug)]
pub struct ControlBindings;

impl BindingTypes for ControlBindings {
    type Axis = PlayerAxisControl;
    type Action = PlayerActionControl;
}
