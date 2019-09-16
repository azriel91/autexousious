use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use character_model::{
    config::CharacterDefinition,
    loaded::{Character, CharacterControlTransitions, CharacterCts},
};
use derive_new::new;
use object_loading::ObjectDefinitionToWrapperProcessor;
use typename::TypeName;

/// Name of the `Processor<Character>` system.
pub const CHARACTER_PROCESSOR: &str = "character_processor";

/// Adds the following processor `System`s to the world:
///
/// * `Processor::<CharacterDefinition>`
/// * `ObjectDefinitionToWrapperProcessor::<Character>`
/// * `Processor::<CharacterCts>`
/// * `Processor::<Character>`
#[derive(Debug, new)]
pub struct CharacterLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<CharacterDefinition>::new(),
            "character_definition_processor",
            &[],
        ); // kcov-ignore
        builder.add(
            ObjectDefinitionToWrapperProcessor::<Character>::new(),
            &ObjectDefinitionToWrapperProcessor::<Character>::type_name(),
            &["character_definition_processor"],
        ); // kcov-ignore
        builder.add(
            Processor::<CharacterControlTransitions>::new(),
            "character_control_transitions_processor",
            &[&ObjectDefinitionToWrapperProcessor::<Character>::type_name()],
        ); // kcov-ignore
        builder.add(
            Processor::<CharacterCts>::new(),
            "character_cts_processor",
            &["character_control_transitions_processor"],
        ); // kcov-ignore
        builder.add(
            Processor::<Character>::new(),
            CHARACTER_PROCESSOR,
            &["character_cts_processor"],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        assets::AssetStorage,
        core::TransformBundle,
        ecs::WorldExt,
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use character_model::{
        config::CharacterDefinition,
        loaded::{Character, CharacterCts, CharacterObjectWrapper},
    };
    use sequence_loading::SequenceLoadingBundle;

    use super::CharacterLoadingBundle;

    #[test]
    fn bundle_build() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<CharacterDefinition>>();
                world.read_resource::<AssetStorage<CharacterObjectWrapper>>();
                world.read_resource::<AssetStorage<CharacterCts>>();
                world.read_resource::<AssetStorage<Character>>();
            })
            .run_isolated()
    }
}
