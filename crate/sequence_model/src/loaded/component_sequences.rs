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
        assets::{AssetStorage, Loader, ProgressCounter},
        ecs::World,
        renderer::{sprite::SpriteSheetHandle, SpriteRender, SpriteSheet, Texture},
        Error,
    };
    use amethyst_test::{AmethystApplication, RenderBaseAppExt};
    use application::{load_in, resource::Format};
    use assets_test::ASSETS_CHAR_BAT_PATH;
    use collision_loading::CollisionLoadingBundle;
    use collision_model::{
        config::{Body, Interactions},
        loaded::{BodySequence, InteractionsSequence},
    };
    use sprite_loading::SpriteLoader;
    use sprite_model::{config::SpritesDefinition, loaded::SpriteRenderSequence};

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
        AmethystApplication::render_base()
            .with_assertion(|world| {
                let component_sequence = sprite_render_sequence(world);

                let component_sequences = ComponentSequences::new(vec![component_sequence]);
                assert_eq!(2, component_sequences.frame_count());
            })
            .run()
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
            .run()
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
            .run()
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
        let sprites_definition = load_in::<SpritesDefinition, _>(
            &*ASSETS_CHAR_BAT_PATH,
            "sprites.toml",
            Format::Toml,
            None,
        )
        .expect("Failed to load sprites_definition.");

        let loader = world.read_resource::<Loader>();
        let texture_assets = world.read_resource::<AssetStorage<Texture>>();
        let sprite_sheet_assets = world.read_resource::<AssetStorage<SpriteSheet>>();

        let sprite_sheet_handles = SpriteLoader::load(
            &mut ProgressCounter::default(),
            &loader,
            &texture_assets,
            &sprite_sheet_assets,
            &sprites_definition,
            &ASSETS_CHAR_BAT_PATH,
        )
        .expect("Failed to load sprites for test.");

        sprite_sheet_handles
            .iter()
            .next()
            .expect("Expected at least one sprite sheet to exist.")
            .clone()
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
