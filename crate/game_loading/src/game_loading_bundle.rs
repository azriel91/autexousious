use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::{
    CharacterAugmentRectifySystem, CharacterSelectionSpawningSystem, MapSelectionSpawningSystem,
};

/// Adds game loading systems to the provided dispatcher.
#[derive(Debug, new)]
pub struct GameLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            CharacterSelectionSpawningSystem::new(),
            any::type_name::<CharacterSelectionSpawningSystem>(),
            &[],
        ); // kcov-ignore
        builder.add(
            CharacterAugmentRectifySystem::new(),
            any::type_name::<CharacterAugmentRectifySystem>(),
            &[
                // Ideally we would also specify `character_prefab::CHARACTER_PREFAB_LOADER_SYSTEM`
                // However, it is in the main dispatcher, so we cannot depend on it.
                any::type_name::<CharacterSelectionSpawningSystem>(),
            ],
        ); // kcov-ignore
        builder.add(
            MapSelectionSpawningSystem::new(),
            any::type_name::<MapSelectionSpawningSystem>(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
