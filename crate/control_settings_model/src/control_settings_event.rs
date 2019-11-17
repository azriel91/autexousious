/// Event signalling a change in the `ControlSettings` state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControlSettingsEvent {
    /// Returns to the previous menu.
    Return,
    /// Control settings should be reloaded.
    ReloadRequest,
}
