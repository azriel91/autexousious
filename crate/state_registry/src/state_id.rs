use derivative::Derivative;

/// Central list of identifiers for `State`s.
#[derive(
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::EnumIter,
    Clone,
    Copy,
    Debug,
    Derivative,
    PartialEq,
)]
#[derivative(Default)]
#[strum(serialize_all = "snake_case")]
pub enum StateId {
    /// No current state.
    ///
    /// Should never be used -- here to allow for `pausable` systems.
    #[derivative(Default)]
    None,
    /// `CharacterSelectionState` ID.
    CharacterSelection,
    /// `ControlSettingsState` ID.
    ControlSettings,
    /// `GameModeSelectionState` ID.
    GameModeSelection,
    /// `GameLoadingState` ID.
    GameLoading,
    /// `GamePlayState` ID.
    GamePlay,
    /// `LoadingState` ID.
    Loading,
    /// `MapSelectionState` ID.
    MapSelection,
}
