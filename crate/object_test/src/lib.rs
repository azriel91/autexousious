#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides support for testing `Object`s.

use amethyst::{
    assets::Processor,
    audio::Source,
    core::TransformBundle,
    renderer::{types::DefaultBackend, RenderEmptyBundle},
    window::ScreenDimensions,
    GameData,
};
use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
use application_event::{AppEvent, AppEventReader};
use collision_loading::CollisionLoadingBundle;
use game_input_model::config::ControlBindings;
use sequence_loading::SequenceLoadingBundle;
use spawn_loading::SpawnLoadingBundle;
use sprite_loading::SpriteLoadingBundle;

pub use crate::object_builder::ObjectBuilder;

mod object_builder;

/// Functions to support tests that use `Object`s.
#[derive(Debug)]
pub struct ObjectTest;

impl ObjectTest {
    /// Returns an `AmethystApplication` with bundles necessary to load an `Object`.
    pub fn application() -> AmethystApplication<GameData<'static, 'static>, AppEvent, AppEventReader>
    {
        AmethystApplication::blank()
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_ui_bundles::<ControlBindings>()
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(SpawnLoadingBundle::new())
    }
}
