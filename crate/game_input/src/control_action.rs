/// Control actions for characters.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
pub enum ControlAction {
    /// Defend button.
    Defend,
    /// Jump button.
    Jump,
    /// Attack button.
    Attack,
    /// "Once off" special attacks or infrequent commands.
    Special,
}

// Required by Amethyst.
impl Default for ControlAction {
    fn default() -> Self {
        ControlAction::Attack
    }
}
