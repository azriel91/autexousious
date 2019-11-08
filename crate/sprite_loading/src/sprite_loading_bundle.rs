use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use sprite_model::{
    config::SpritesDefinition,
    loaded::{SpriteRenderSequence, TintSequence},
};

/// Adds the following systems to the dispatcher:
///
/// * `Processor::<SpritesDefinition>`
/// * `Processor::<SpriteRenderSequence>`
/// * `Processor::<TintSequence>`
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
        builder.add(
            Processor::<TintSequence>::new(),
            "tint_sequence_processor",
            &["sprites_definition_processor"],
        );
        Ok(())
    }
}
