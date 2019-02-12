use amethyst::{assets::Processor, core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use derive_new::new;
use sprite_model::config::SpritesDefinition;

/// Adds the `Processor::<SpritesDefinition>` system to the world.
#[derive(Debug, new)]
pub struct SpriteLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SpriteLoadingBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            Processor::<SpritesDefinition>::new(),
            "sprites_definition_processor",
            &[],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, Error};
    use amethyst_test::AmethystApplication;
    use sprite_model::config::SpritesDefinition;

    use super::SpriteLoadingBundle;

    #[test]
    fn bundle_build_adds_sprite_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(SpriteLoadingBundle)
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<SpritesDefinition>>();
            })
            .run()
    }
}
