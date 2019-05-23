use amethyst::{
    assets::{PrefabLoaderSystem, Processor},
    core::bundle::SystemBundle,
    ecs::DispatcherBuilder,
    Error,
};
use character_model::{
    config::CharacterDefinition,
    loaded::{Character, CharacterControlTransitions, CharacterControlTransitionsSequence},
};
use derive_new::new;
use object_loading::ObjectDefinitionToWrapperProcessor;
use typename::TypeName;

use crate::CharacterPrefab;

/// Adds the following processor `System`s to the world:
///
/// * `Processor::<CharacterDefinition>`
/// * `ObjectDefinitionToWrapperProcessor::<Character>`
/// * `Processor::<CharacterControlTransitionsSequence>`
/// * `Processor::<Character>`
/// * `PrefabLoaderSystem::<CharacterPrefab>`
#[derive(Debug, new)]
pub struct CharacterLoadingBundle;

/// Name of the `CharacterPrefab` `PrefabLoaderSystem`.
pub const CHARACTER_PREFAB_LOADER_SYSTEM: &str = "character_prefab_loader_system";

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterLoadingBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            Processor::<CharacterDefinition>::new(),
            "character_definition_processor",
            &["component_sequences_processor"],
        ); // kcov-ignore
        builder.add(
            ObjectDefinitionToWrapperProcessor::<Character>::new(),
            &ObjectDefinitionToWrapperProcessor::<Character>::type_name(),
            &["character_definition_processor", "sprite_sheet_processor"],
        ); // kcov-ignore
        builder.add(
            Processor::<CharacterControlTransitions>::new(),
            "character_control_transitions_processor",
            &[&ObjectDefinitionToWrapperProcessor::<Character>::type_name()],
        ); // kcov-ignore
        builder.add(
            Processor::<CharacterControlTransitionsSequence>::new(),
            "character_control_transitions_sequence_processor",
            &["character_control_transitions_processor"],
        ); // kcov-ignore
        builder.add(
            Processor::<Character>::new(),
            "character_processor",
            &["character_control_transitions_sequence_processor"],
        ); // kcov-ignore
        builder.add(
            PrefabLoaderSystem::<CharacterPrefab>::default(),
            CHARACTER_PREFAB_LOADER_SYSTEM,
            &["character_processor"],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, Error};
    use amethyst_test::{AmethystApplication, RenderBaseAppExt};
    use character_model::{
        config::CharacterDefinition,
        loaded::{Character, CharacterControlTransitionsSequence, CharacterObjectWrapper},
    };
    use sequence_loading::SequenceLoadingBundle;

    use super::CharacterLoadingBundle;

    #[test]
    fn bundle_build() -> Result<(), Error> {
        AmethystApplication::render_base()
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<CharacterDefinition>>();
                world.read_resource::<AssetStorage<CharacterObjectWrapper>>();
                world.read_resource::<AssetStorage<CharacterControlTransitionsSequence>>();
                world.read_resource::<AssetStorage<Character>>();
            })
            .run()
    }
}
