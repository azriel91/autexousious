/// Game mode menu indicies.
#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum GameModeIndex {
    /// Menu item for starting a game.
    StartGame,
    /// Menu item for exiting the application.
    Exit,
}
