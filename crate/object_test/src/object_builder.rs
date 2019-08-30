use std::marker::PhantomData;

use amethyst::{
    assets::AssetStorage,
    ecs::{Read, World},
    renderer::{
        loaders::load_from_srgba, palette::Srgba, types::TextureData, Sprite, SpriteRender,
        SpriteSheet, Texture,
    },
};
use collision_model::{
    config::{Body, Interactions},
    loaded::{
        BodySequence, BodySequenceHandles, InteractionsSequence, InteractionsSequenceHandles,
    },
};
use derive_new::new;
use kinematic_model::{
    config::ObjectAcceleration,
    loaded::{ObjectAccelerationSequence, ObjectAccelerationSequenceHandles},
};
use object_loading::ObjectLoaderSystemData;
use object_model::loaded::{GameObject, Object, ObjectWrapper};
use sequence_model::{
    config::Wait,
    loaded::{SequenceEndTransition, SequenceEndTransitions, WaitSequence, WaitSequenceHandles},
};
use spawn_model::{
    config::Spawns,
    loaded::{SpawnsSequence, SpawnsSequenceHandles},
};
use sprite_model::loaded::{SpriteRenderSequence, SpriteRenderSequenceHandles};

/// Builds an `Object` in-memory.
///
/// This defaults to the minimal fields necessary for an object. Builder methods can be used to
/// override the defaults.
#[derive(Debug, new)]
pub struct ObjectBuilder<O>
where
    O: GameObject,
{
    /// `Wait` to use in all frames.
    #[new(value = "Wait::new(2)")]
    pub wait: Wait,
    /// `ObjectAcceleration` to use in all frames.
    #[new(default)]
    pub object_acceleration: ObjectAcceleration,
    /// `Body` to use in all frames.
    #[new(default)]
    pub body: Body,
    /// `Interactions` to use in all frames.
    #[new(default)]
    pub interactions: Interactions,
    /// `Spawns` to use in all frames.
    #[new(default)]
    pub spawns: Spawns,
    /// Marker.
    pub marker: PhantomData<O>,
}

impl<O> ObjectBuilder<O>
where
    O: GameObject,
{
    /// Set the `Wait` for this `Object`.
    pub fn with_wait(mut self, wait: Wait) -> Self {
        self.wait = wait;
        self
    }

    /// Set the `ObjectAcceleration` for this `Object`.
    pub fn with_object_acceleration(mut self, object_acceleration: ObjectAcceleration) -> Self {
        self.object_acceleration = object_acceleration;
        self
    }

    /// Set the `Body` for this `Object`.
    pub fn with_body(mut self, body: Body) -> Self {
        self.body = body;
        self
    }

    /// Set the `Interactions` for this `Object`.
    pub fn with_interactions(mut self, interactions: Interactions) -> Self {
        self.interactions = interactions;
        self
    }

    /// Set the `Spawns` for this `Object`.
    pub fn with_spawns(mut self, spawns: Spawns) -> Self {
        self.spawns = spawns;
        self
    }

    /// Builds and returns the object.
    pub fn build(self, world: &World) -> Object {
        let (
            wait_sequence_handles,
            object_acceleration_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            spawns_sequence_handles,
        ) = {
            let (
                ObjectLoaderSystemData {
                    loader,
                    wait_sequence_assets,
                    object_acceleration_sequence_assets,
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

            let wait_sequence = WaitSequence::new(vec![self.wait]);

            let object_acceleration_sequence =
                ObjectAccelerationSequence::new(vec![self.object_acceleration]);

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

            let body_handle = loader.load_from_data(self.body.clone(), (), &body_assets);
            let body_sequence = BodySequence::new(vec![body_handle]);

            let interactions_handle =
                loader.load_from_data(self.interactions.clone(), (), &interactions_assets);
            let interactions_sequence = InteractionsSequence::new(vec![interactions_handle]);

            let spawns_handle = loader.load_from_data(self.spawns.clone(), (), &spawns_assets);
            let spawns_sequence = SpawnsSequence::new(vec![spawns_handle]);

            let wait_sequence_handle =
                loader.load_from_data(wait_sequence, (), &wait_sequence_assets);
            let object_acceleration_sequence_handle = loader.load_from_data(
                object_acceleration_sequence,
                (),
                &object_acceleration_sequence_assets,
            );
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
                mut object_acceleration_sequence_handles,
                mut sprite_render_sequence_handles,
                mut body_sequence_handles,
                mut interactions_sequence_handles,
                mut spawns_sequence_handles,
            ) = (
                WaitSequenceHandles::default(),
                ObjectAccelerationSequenceHandles::default(),
                SpriteRenderSequenceHandles::default(),
                BodySequenceHandles::default(),
                InteractionsSequenceHandles::default(),
                SpawnsSequenceHandles::default(),
            );
            wait_sequence_handles.push(wait_sequence_handle);
            object_acceleration_sequence_handles.push(object_acceleration_sequence_handle);
            sprite_render_sequence_handles.push(sprite_render_sequence_handle);
            body_sequence_handles.push(body_sequence_handle);
            interactions_sequence_handles.push(interactions_sequence_handle);
            spawns_sequence_handles.push(spawns_sequence_handle);

            (
                wait_sequence_handles,
                object_acceleration_sequence_handles,
                sprite_render_sequence_handles,
                body_sequence_handles,
                interactions_sequence_handles,
                spawns_sequence_handles,
            )
        };
        let sequence_end_transitions = {
            let mut sequence_end_transitions = Vec::default();
            sequence_end_transitions.push(SequenceEndTransition::None);
            SequenceEndTransitions::new(sequence_end_transitions)
        };
        Object::new(
            wait_sequence_handles,
            object_acceleration_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            spawns_sequence_handles,
            sequence_end_transitions,
        )
    }

    /// Builds and returns the object wrapper.
    pub fn build_wrapper(self, world: &World) -> O::ObjectWrapper {
        O::ObjectWrapper::new(self.build(world))
    }
}
