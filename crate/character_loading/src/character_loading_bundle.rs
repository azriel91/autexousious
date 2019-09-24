use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use character_model::{
    config::CharacterDefinition,
    loaded::{CharacterControlTransitions, CharacterCts},
};
use derive_new::new;

/// Name of the `Processor<Character>` system.
pub const CHARACTER_PROCESSOR: &str = "character_processor";

/// Adds the following processor `System`s to the world:
///
/// * `Processor::<CharacterDefinition>`
/// * `Processor::<CharacterControlTransitions>`
/// * `Processor::<CharacterCts>`
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
            Processor::<CharacterControlTransitions>::new(),
            "character_control_transitions_processor",
            &["character_definition_processor"],
        ); // kcov-ignore
        builder.add(
            Processor::<CharacterCts>::new(),
            "character_cts_processor",
            &["character_control_transitions_processor"],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use character_model::{config::CharacterDefinition, loaded::CharacterCts};

    use super::CharacterLoadingBundle;

    #[test]
    fn bundle_build() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(CharacterLoadingBundle::new())
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<CharacterDefinition>>();
                world.read_resource::<AssetStorage<CharacterCts>>();
            })
            .run()
    }
}
