use std::collections::HashMap;

use amethyst::input::{Axis as InputAxis, Button};

use Axis;
use ControlAction;

/// Structure for each controller's configuration.
#[derive(Clone, Derivative, Default, PartialEq, Serialize, Deserialize, new)]
#[derivative(Debug)]
pub struct ControllerConfig {
    /// Axis control configuration.
    pub axes: HashMap<Axis, InputAxis>, // kcov-ignore
    /// Action control configuration.
    pub actions: HashMap<ControlAction, Button>, // kcov-ignore
}
