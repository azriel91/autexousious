use std::collections::HashMap;

use amethyst::input::{Axis as InputAxis, Button as InputButton};

use Axis;
use ControlAction;

/// Structure for each controller's configuration.
// TODO: `PartialEq` pending <https://github.com/amethyst/amethyst/pull/904>
#[derive(Clone, Derivative, Default, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct ControllerConfig {
    /// Axis control configuration.
    // TODO: Pending <https://github.com/amethyst/amethyst/pull/904>
    #[derivative(Debug = "ignore")]
    pub axes: HashMap<Axis, InputAxis>,
    /// Action control configuration.
    pub actions: HashMap<ControlAction, InputButton>,
}
