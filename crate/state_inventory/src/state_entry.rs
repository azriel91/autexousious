use derive_new::new;

/// Inventory entry for a `State`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, new)]
pub struct StateEntry {
    /// The name of the crate, such as `"game_play"`.
    pub crate_name: &'static str,
    /// The name of the `State`, such as `"GamePlayState"`.
    pub name: &'static str,
}
