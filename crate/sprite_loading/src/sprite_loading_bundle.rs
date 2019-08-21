use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use sprite_model::{config::SpritesDefinition, loaded::SpriteRenderSequence};

/// Adds the following systems to the dispatcher:
///
/// * `Processor::<SpritesDefinition>`
/// * `Processor::<SpriteRenderSequence>`
#[derive(Debug, new)]
pub struct SpriteLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SpriteLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<SpritesDefinition>::new(),
            "sprites_definition_processor",
            &[],
        );
        builder.add(
            Processor::<SpriteRenderSequence>::new(),
            "sprite_render_sequence_processor",
            &["sprites_definition_processor"],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use sprite_model::{config::SpritesDefinition, loaded::SpriteRenderSequence};

    use super::SpriteLoadingBundle;

    #[test]
    fn bundle_build_adds_sprite_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(SpriteLoadingBundle)
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<SpritesDefinition>>();
                world.read_resource::<AssetStorage<SpriteRenderSequence>>();
            })
            .run()
    }
}
