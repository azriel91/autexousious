#[cfg(test)]
mod tests {
    use game_loading::{CharacterAugmentStatus, GameLoadingStatus};

    #[test]
    fn loaded_is_false_when_map_and_characters_not_loaded() {
        let status = GameLoadingStatus::new();

        assert!(!status.loaded())
    }

    #[test]
    fn loaded_is_false_when_map_not_loaded() {
        let mut status = GameLoadingStatus::new();
        status.character_augment_status = CharacterAugmentStatus::Complete;

        assert!(!status.loaded())
    }

    #[test]
    fn loaded_is_false_when_characters_not_loaded() {
        let mut status = GameLoadingStatus::new();
        status.map_loaded = true;

        assert!(!status.loaded())
    }

    #[test]
    fn loaded_is_true_when_map_and_character_augment_status() {
        let mut status = GameLoadingStatus::new();
        status.map_loaded = true;
        status.character_augment_status = CharacterAugmentStatus::Complete;

        assert!(status.loaded())
    }

    #[test]
    fn reset_sets_all_fields_to_false() {
        let mut status = GameLoadingStatus::new();
        status.map_loaded = true;
        status.character_augment_status = CharacterAugmentStatus::Complete;
        status.reset();

        assert!(!status.map_loaded);
        assert_eq!(
            CharacterAugmentStatus::Prefab,
            status.character_augment_status
        );
    }
}
