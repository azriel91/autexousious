#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides support for testing `Object`s.

use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Processor},
    audio::Source,
    core::TransformBundle,
    ecs::{Read, World},
    renderer::{
        loaders::load_from_srgba,
        palette::Srgba,
        types::{DefaultBackend, TextureData},
        RenderEmptyBundle, Sprite, SpriteRender, SpriteSheet, Texture,
    },
    window::ScreenDimensions,
    GameData,
};
use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
use application_event::{AppEvent, AppEventReader};
use collision_loading::CollisionLoadingBundle;
use collision_model::{
    config::{Body, Hit, Interaction, InteractionKind, Interactions},
    loaded::{BodySequence, BodySequenceHandle, InteractionsSequence, InteractionsSequenceHandle},
};
use fnv::FnvHashMap;
use game_input_model::ControlBindings;
use object_loading::ObjectLoaderSystemData;
use object_model::loaded::{GameObject, Object, ObjectWrapper};
use sequence_loading::SequenceLoadingBundle;
use sequence_model::{
    config::Wait,
    loaded::{SequenceEndTransition, SequenceEndTransitions, WaitSequence, WaitSequenceHandle},
};
use shape_model::Volume;
use spawn_loading::SpawnLoadingBundle;
use spawn_model::{
    config::Spawns,
    loaded::{SpawnsSequence, SpawnsSequenceHandle},
};
use sprite_loading::SpriteLoadingBundle;
use sprite_model::loaded::{SpriteRenderSequence, SpriteRenderSequenceHandle};

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

    /// Returns an `O::ObjectWrapper` for an in-memory initialized object.
    pub fn object_wrapper<O>(world: &mut World) -> O::ObjectWrapper
    where
        O: GameObject,
    {
        let (
            wait_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            spawns_sequence_handles,
        ) = {
            let (
                ObjectLoaderSystemData {
                    loader,
                    wait_sequence_assets,
                    sprite_render_sequence_assets,
                    body_sequence_assets,
                    interactions_sequence_assets,
                    spawns_sequence_assets,
                    body_assets,
                    interactions_assets,
                    spawns_assets,
                },
                texture_assets,
                sprite_sheet_assets,
            ) = world.system_data::<(
                ObjectLoaderSystemData<'_>,
                Read<'_, AssetStorage<Texture>>,
                Read<'_, AssetStorage<SpriteSheet>>,
            )>();

            let wait_sequence = WaitSequence::new(vec![Wait::new(2)]);

            let texture_builder = load_from_srgba(Srgba::new(0., 0., 0., 1.));
            let texture_data = TextureData::from(texture_builder);
            let texture_handle = loader.load_from_data(texture_data, (), &texture_assets);
            let sprite_sheet = SpriteSheet {
                texture: texture_handle,
                sprites: vec![Sprite::from((
                    (19., 29.),
                    [-9.5, -14.5],
                    [0.5 / 20., 18.5 / 20., 28.5 / 30., 0.5 / 30.],
                ))],
            };
            let sprite_sheet_handle = loader.load_from_data(sprite_sheet, (), &sprite_sheet_assets);
            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle,
                sprite_number: 0,
            };
            let sprite_render_sequence = SpriteRenderSequence::new(vec![sprite_render]);

            let body_handle = loader.load_from_data(body(), (), &body_assets);
            let body_sequence = BodySequence::new(vec![body_handle]);

            let interactions_handle =
                loader.load_from_data(interactions(), (), &interactions_assets);
            let interactions_sequence = InteractionsSequence::new(vec![interactions_handle]);

            let spawns_handle = loader.load_from_data(Spawns::default(), (), &spawns_assets);
            let spawns_sequence = SpawnsSequence::new(vec![spawns_handle]);

            let wait_sequence_handle =
                loader.load_from_data(wait_sequence, (), &wait_sequence_assets);
            let sprite_render_sequence_handle =
                loader.load_from_data(sprite_render_sequence, (), &sprite_render_sequence_assets);
            let body_sequence_handle =
                loader.load_from_data(body_sequence, (), &body_sequence_assets);
            let interactions_sequence_handle =
                loader.load_from_data(interactions_sequence, (), &interactions_sequence_assets);
            let spawns_sequence_handle =
                loader.load_from_data(spawns_sequence, (), &spawns_sequence_assets);

            let (
                mut wait_sequence_handles,
                mut sprite_render_sequence_handles,
                mut body_sequence_handles,
                mut interactions_sequence_handles,
                mut spawns_sequence_handles,
            ) = (
                HashMap::<O::SequenceId, WaitSequenceHandle>::new(),
                HashMap::<O::SequenceId, SpriteRenderSequenceHandle>::new(),
                HashMap::<O::SequenceId, BodySequenceHandle>::new(),
                HashMap::<O::SequenceId, InteractionsSequenceHandle>::new(),
                HashMap::<O::SequenceId, SpawnsSequenceHandle>::new(),
            );
            wait_sequence_handles.insert(O::SequenceId::default(), wait_sequence_handle);
            sprite_render_sequence_handles
                .insert(O::SequenceId::default(), sprite_render_sequence_handle);
            body_sequence_handles.insert(O::SequenceId::default(), body_sequence_handle);
            interactions_sequence_handles
                .insert(O::SequenceId::default(), interactions_sequence_handle);
            spawns_sequence_handles.insert(O::SequenceId::default(), spawns_sequence_handle);

            (
                wait_sequence_handles,
                sprite_render_sequence_handles,
                body_sequence_handles,
                interactions_sequence_handles,
                spawns_sequence_handles,
            )
        };
        let sequence_end_transitions = {
            let mut sequence_end_transitions = FnvHashMap::default();
            sequence_end_transitions
                .insert(O::SequenceId::default(), SequenceEndTransition::new(None));
            SequenceEndTransitions(sequence_end_transitions)
        };
        let object = Object::new(
            wait_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            spawns_sequence_handles,
            sequence_end_transitions,
        );

        O::ObjectWrapper::new(object)
    }
}

fn interactions() -> Interactions {
    Interactions::new(vec![interaction()])
}

fn interaction() -> Interaction {
    Interaction::new(InteractionKind::Hit(Hit::default()), vec![volume()], true)
}

fn body() -> Body {
    Body::new(vec![volume()])
}

fn volume() -> Volume {
    Volume::Box {
        x: 0,
        y: 0,
        z: 0,
        w: 1,
        h: 1,
        d: 1,
    }
}
