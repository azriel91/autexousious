use game_input::ControllerInput;
use mirrored_model::play::Mirrored;

/// Utility functions for checking if game input matches the direction the character is facing.
///
/// New consumers should prefer functions on `game_input_model::InputDirection` instead.
#[derive(Debug)]
pub struct SequenceHandlerUtil;

impl SequenceHandlerUtil {
    /// Returns whether X axis input is in the same direction as the object is facing.
    ///
    /// This returns `false` if there is no input on the X axis.
    ///
    /// # Parameters
    ///
    /// * `controller_input`: Controller input for the character.
    /// * `mirrored`: Whether the character is facing left.
    pub fn input_matches_direction(controller_input: &ControllerInput, mirrored: Mirrored) -> bool {
        controller_input.x_axis_value > 0. && !mirrored.0
            || controller_input.x_axis_value < 0. && mirrored.0
    }

    /// Returns whether X axis input is in the opposite direction as the object is facing.
    ///
    /// This returns `false` if there is no input on the X axis.
    ///
    /// # Parameters
    ///
    /// * `controller_input`: Controller input for the character.
    /// * `mirrored`: Whether the character is facing left.
    pub fn input_opposes_direction(controller_input: &ControllerInput, mirrored: Mirrored) -> bool {
        controller_input.x_axis_value > 0. && mirrored.0
            || controller_input.x_axis_value < 0. && !mirrored.0
    }
}
