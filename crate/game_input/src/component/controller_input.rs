use amethyst::ecs::prelude::*;

/// Input for a character entity.
#[derive(Clone, Copy, Debug, Default, PartialEq, new)]
pub struct ControllerInput {
    /// X axis input value.
    ///
    /// * 0.0 if neither or both left and right buttons are pressed.
    /// * -1.0 if only left button is pressed.
    /// * 1.0 if only right button is pressed.
    pub x_axis_value: f64,
    /// Z axis input value.
    ///
    /// * 0.0 if neither or both up and down buttons are pressed.
    /// * -1.0 if only up button is pressed.
    /// * 1.0 if only down button is pressed.
    pub z_axis_value: f64,
    /// Whether the `Defend` button is pressed.
    pub defend: bool,
    /// Whether the `Jump` button is pressed.
    pub jump: bool,
    /// Whether the `Attack` button is pressed.
    pub attack: bool,
    /// Whether the `Special` button is pressed.
    pub special: bool,
}

impl Component for ControllerInput {
    type Storage = VecStorage<Self>;
}
