use amethyst::{assets::Processor, core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use character_model::{config::CharacterDefinition, loaded::Character};
use derive_new::new;
use object_loading::ObjectDefinitionToWrapperProcessor;
use typename::TypeName;

/// Adds the following processor `System`s to the world:
///
/// * `ObjectDefinitionToWrapperProcessor::<Character>`
/// * `Processor::<Character>`
#[derive(Debug, new)]
pub struct CharacterLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterLoadingBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            ObjectDefinitionToWrapperProcessor::<Character>::new(),
            &ObjectDefinitionToWrapperProcessor::<Character>::type_name(),
            &[],
        );
        builder.add(Processor::<Character>::new(), "character_processor", &[]);
        builder.add(
            Processor::<CharacterDefinition>::new(),
            "character_definition_processor",
            &[],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::assets::AssetStorage;
    use amethyst_test::AmethystApplication;
    use character_model::{
        config::CharacterDefinition,
        loaded::{Character, CharacterObjectWrapper},
    };

    use super::CharacterLoadingBundle;

    #[test]
    fn bundle_build_adds_character_processor() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(CharacterLoadingBundle)
                .with_assertion(|world| {
                    // Panics if the Processors are not added.
                    world.read_resource::<AssetStorage<Character>>();
                    world.read_resource::<AssetStorage<CharacterObjectWrapper>>();
                    world.read_resource::<AssetStorage<CharacterDefinition>>();
                })
                .run()
                .is_ok()
        );
    }
}
