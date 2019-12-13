use game_input::ControllerInput;
use mirrored_model::play::Mirrored;

/// Parameters to check if a `BasicIrr` is met.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BasicIrrParams {
    /// `ControllerInput` of the entity.
    pub controller_input: Option<ControllerInput>,
    /// `Mirrored` of the entity.
    pub mirrored: Option<Mirrored>,
}
