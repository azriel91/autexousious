use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use typename::TypeName;

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
            &CharacterSelectionSpawningSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            CharacterAugmentRectifySystem::new(),
            &CharacterAugmentRectifySystem::type_name(),
            &[
                // Ideally we would also specify `character_prefab::CHARACTER_PREFAB_LOADER_SYSTEM`
                // However, it is in the main dispatcher, so we cannot depend on it.
                &CharacterSelectionSpawningSystem::type_name(),
            ],
        ); // kcov-ignore
        builder.add(
            MapSelectionSpawningSystem::new(),
            &MapSelectionSpawningSystem::type_name(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
