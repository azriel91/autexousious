use derive_new::new;

use crate::CharacterAugmentStatus;

/// Status of setting up entities for game play.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, new)]
pub struct GameLoadingStatus {
    /// Whether the map is loaded.
    #[new(default)]
    pub map_loaded: bool,
    /// Whether characters are loaded.
    #[new(default)]
    pub character_augment_status: CharacterAugmentStatus,
}

impl GameLoadingStatus {
    /// Returns whether all parts of game loading have been completed.
    pub fn loaded(self) -> bool {
        self.map_loaded && self.character_augment_status == CharacterAugmentStatus::Complete
    }

    /// Sets all parts of this status to false.
    pub fn reset(&mut self) {
        *self = GameLoadingStatus::new();
    }
}
