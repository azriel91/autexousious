/// Central list of identifiers for `State`s.
#[derive(
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::EnumIter,
    Clone,
    Copy,
    Debug,
    PartialEq,
)]
#[strum(serialize_all = "snake_case")]
pub enum StateId {
    /// `CharacterSelectionState` ID.
    CharacterSelection,
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
