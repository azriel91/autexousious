use derive_new::new;

/// Status of setting up entities for game play.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, new)]
pub struct GameLoadingStatus {
    /// Whether the map is loaded.
    #[new(default)]
    pub map_loaded: bool,
    /// Whether characters are loaded.
    #[new(default)]
    pub characters_loaded: bool,
}

impl GameLoadingStatus {
    /// Returns whether all parts of game loading have been completed.
    pub fn loaded(self) -> bool {
        self.map_loaded && self.characters_loaded
    }

    /// Sets all parts of this status to false.
    pub fn reset(&mut self) {
        *self = GameLoadingStatus::new();
    }
}

#[cfg(test)]
mod tests {
    use super::GameLoadingStatus;

    #[test]
    fn loaded_is_false_when_map_and_characters_not_loaded() {
        let status = GameLoadingStatus::new();

        assert!(!status.loaded())
    }

    #[test]
    fn loaded_is_false_when_map_not_loaded() {
        let mut status = GameLoadingStatus::new();
        status.characters_loaded = true;

        assert!(!status.loaded())
    }

    #[test]
    fn loaded_is_false_when_characters_not_loaded() {
        let mut status = GameLoadingStatus::new();
        status.map_loaded = true;

        assert!(!status.loaded())
    }

    #[test]
    fn loaded_is_true_when_map_and_characters_loaded() {
        let mut status = GameLoadingStatus::new();
        status.map_loaded = true;
        status.characters_loaded = true;

        assert!(status.loaded())
    }

    #[test]
    fn reset_sets_all_fields_to_false() {
        let mut status = GameLoadingStatus::new();
        status.map_loaded = true;
        status.characters_loaded = true;
        status.reset();

        assert!(!status.map_loaded);
        assert!(!status.characters_loaded);
    }
}
