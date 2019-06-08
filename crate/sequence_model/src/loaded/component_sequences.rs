use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::storage::VecStorage,
    Error,
};
use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::loaded::ComponentSequence;

/// Newtype for `Vec<ComponentSequence>`.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct ComponentSequences(
    /// The underlying vector.
    pub Vec<ComponentSequence>,
);

/// Handle to a `ComponentSequences` asset.
pub type ComponentSequencesHandle = Handle<ComponentSequences>;

impl Asset for ComponentSequences {
    const NAME: &'static str = concat!(module_path!(), "::", stringify!(ComponentSequences));
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<ComponentSequences> for Result<ProcessingState<ComponentSequences>, Error> {
    fn from(
        component_sequences: ComponentSequences,
    ) -> Result<ProcessingState<ComponentSequences>, Error> {
        Ok(ProcessingState::Loaded(component_sequences))
    }
}

impl ComponentSequences {
    /// Returns the number of frames in the component sequences.
    pub fn frame_count(&self) -> usize {
        self.0
            .iter()
            .next()
            .map(|component_sequence| match component_sequence {
                ComponentSequence::Wait(sequence) => sequence.len(),
                ComponentSequence::SpriteRender(sequence) => sequence.len(),
                ComponentSequence::Body(sequence) => sequence.len(),
                ComponentSequence::Interactions(sequence) => sequence.len(),
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::{AssetStorage, Loader},
        core::TransformBundle,
        ecs::World,
        renderer::{
            loaders::load_from_srgba,
            palette::Srgba,
            sprite::{Sprite, SpriteSheetHandle},
            types::{DefaultBackend, TextureData},
            RenderEmptyBundle, SpriteRender, SpriteSheet, Texture,
        },
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_loading::CollisionLoadingBundle;
    use collision_model::{
        config::{Body, Interactions},
        loaded::{BodySequence, InteractionsSequence},
    };
    use sprite_model::loaded::SpriteRenderSequence;

    use super::ComponentSequences;
    use crate::{
        config::Wait,
        loaded::{ComponentSequence, WaitSequence},
    };

    #[test]
    fn frame_count_defaults_to_zero() {
        assert_eq!(0, ComponentSequences::new(Vec::new()).frame_count())
    }

    #[test]
    fn frame_count_returns_sequence_length_for_wait() {
        let component_sequences = ComponentSequences::new(vec![wait_sequence()]);
        assert_eq!(2, component_sequences.frame_count());
    }

    // TODO: Expose functionality for testing: <https://github.com/amethyst/amethyst/issues/1438>

    #[test]
    fn frame_count_returns_sequence_length_for_sprite_render() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_assertion(|world| {
                let component_sequence = sprite_render_sequence(world);

                let component_sequences = ComponentSequences::new(vec![component_sequence]);
                assert_eq!(2, component_sequences.frame_count());
            })
            .run_isolated()
    }

    #[test]
    fn frame_count_returns_sequence_length_for_body() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(CollisionLoadingBundle::new())
            .with_assertion(|world| {
                let component_sequence = body_sequence(world);

                let component_sequences = ComponentSequences::new(vec![component_sequence]);
                assert_eq!(2, component_sequences.frame_count());
            })
            .run_isolated()
    }

    #[test]
    fn frame_count_returns_sequence_length_for_interactions() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(CollisionLoadingBundle::new())
            .with_assertion(|world| {
                let component_sequence = interactions_sequence(world);

                let component_sequences = ComponentSequences::new(vec![component_sequence]);
                assert_eq!(2, component_sequences.frame_count());
            })
            .run_isolated()
    }

    fn wait_sequence() -> ComponentSequence {
        let sequences = vec![Wait::new(12), Wait::new(32)];
        ComponentSequence::Wait(WaitSequence::new(sequences))
    }

    fn sprite_render_sequence(world: &mut World) -> ComponentSequence {
        let sprite_render = sprite_render(world);

        let sequences = vec![sprite_render.clone(), sprite_render];
        ComponentSequence::SpriteRender(SpriteRenderSequence::new(sequences))
    }

    fn sprite_render(world: &mut World) -> SpriteRender {
        let sprite_sheet = sprite_sheet_handle(world);
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    }

    fn sprite_sheet_handle(world: &mut World) -> SpriteSheetHandle {
        let loader = world.read_resource::<Loader>();
        let texture_assets = world.read_resource::<AssetStorage<Texture>>();
        let sprite_sheet_assets = world.read_resource::<AssetStorage<SpriteSheet>>();

        let texture_builder = load_from_srgba(Srgba::new(0., 0., 0., 1.));
        let texture_data = TextureData::from(texture_builder);
        let texture_handle = loader.load_from_data(texture_data, (), &texture_assets);
        let sprite = Sprite::from_pixel_values(200, 100, 20, 10, 0, 0, [0.; 2], false, false);
        let sprites = vec![sprite];
        let sprite_sheet = SpriteSheet {
            texture: texture_handle,
            sprites,
        };

        loader.load_from_data(sprite_sheet, (), &sprite_sheet_assets)
    }

    fn body_sequence(world: &mut World) -> ComponentSequence {
        let loader = world.read_resource::<Loader>();
        let handle = loader.load_from_data(Body::default(), (), &world.read_resource());
        let sequences = vec![handle.clone(), handle];
        ComponentSequence::Body(BodySequence::new(sequences))
    }

    fn interactions_sequence(world: &mut World) -> ComponentSequence {
        let loader = world.read_resource::<Loader>();
        let handle = loader.load_from_data(Interactions::default(), (), &world.read_resource());
        let sequences = vec![handle.clone(), handle];
        ComponentSequence::Interactions(InteractionsSequence::new(sequences))
    }
}
