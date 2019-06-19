use amethyst::{core::transform::Transform, ecs::Entity, renderer::transparent::Transparent};
use kinematic_model::config::{Position, Velocity};
use logic_clock::LogicClock;
use object_model::{loaded::ObjectWrapper, play::Mirrored};
use sequence_model::play::{FrameIndexClock, FrameWaitClock, SequenceStatus};

use crate::ObjectComponentStorages;

/// Placeholder constant for
const UNINITIALIZED: usize = 99;

/// Augments an entity with `Object` components.
#[derive(Debug)]
pub struct ObjectEntityAugmenter;

impl ObjectEntityAugmenter {
    /// Augments an entity with `Object` components.
    ///
    /// # Parameters
    ///
    /// * `entity`: The entity to augment.
    /// * `object_component_storages`: Non-frame-dependent `Component` storages for objects.
    /// * `object_wrapper`: Slug and handle of the object to spawn.
    pub fn augment<'s, W>(
        entity: Entity,
        ObjectComponentStorages {
            ref mut transparents,
            ref mut positions,
            ref mut velocities,
            ref mut transforms,
            ref mut mirroreds,
            ref mut sequence_end_transitionses,
            ref mut sequence_ids,
            ref mut sequence_statuses,
            ref mut frame_index_clocks,
            ref mut frame_wait_clocks,
        }: &mut ObjectComponentStorages<'s, W::SequenceId>,
        object_wrapper: &W,
    ) where
        W: ObjectWrapper,
    {
        let sequence_end_transitions = &object_wrapper.inner().sequence_end_transitions;

        let sequence_id = W::SequenceId::default();

        transparents
            .insert(entity, Transparent)
            .expect("Failed to insert transparent component.");
        positions
            .insert(entity, Position::default())
            .expect("Failed to insert position component.");
        velocities
            .insert(entity, Velocity::default())
            .expect("Failed to insert velocity component.");
        transforms
            .insert(entity, Transform::default())
            .expect("Failed to insert transform component.");
        mirroreds
            .insert(entity, Mirrored::default())
            .expect("Failed to insert mirrored component.");
        sequence_end_transitionses
            .insert(entity, sequence_end_transitions.clone())
            .expect("Failed to insert sequence_end_transitions component.");
        sequence_ids
            .insert(entity, sequence_id)
            .expect("Failed to insert sequence_id component.");
        sequence_statuses
            .insert(entity, SequenceStatus::default())
            .expect("Failed to insert sequence_status component.");
        frame_index_clocks
            .insert(entity, FrameIndexClock::new(LogicClock::new(UNINITIALIZED)))
            .expect("Failed to insert frame_index_clock component.");
        frame_wait_clocks
            .insert(entity, FrameWaitClock::new(LogicClock::new(UNINITIALIZED)))
            .expect("Failed to insert frame_wait_clock component.");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use amethyst::{
        assets::{AssetStorage, Processor},
        audio::Source,
        core::{transform::Transform, TransformBundle},
        ecs::{Builder, Read, SystemData, World},
        renderer::{
            loaders::load_from_srgba,
            palette::Srgba,
            transparent::Transparent,
            types::{DefaultBackend, TextureData},
            RenderEmptyBundle, Sprite, SpriteRender, SpriteSheet, Texture,
        },
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use application_event::{AppEvent, AppEventReader};
    use collision_loading::CollisionLoadingBundle;
    use collision_model::{
        config::{Body, Hit, Interaction, InteractionKind, Interactions},
        loaded::{
            BodySequence, BodySequenceHandle, InteractionsSequence, InteractionsSequenceHandle,
        },
    };
    use fnv::FnvHashMap;
    use game_input_model::ControlBindings;
    use kinematic_model::config::{Position, Velocity};
    use object_loading::ObjectLoaderSystemData;
    use object_model::{loaded::Object, play::Mirrored};
    use sequence_loading::SequenceLoadingBundle;
    use sequence_model::{
        config::Wait,
        loaded::{SequenceEndTransition, SequenceEndTransitions, WaitSequence, WaitSequenceHandle},
        play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
    };
    use shape_model::Volume;
    use spawn_model::{
        config::Spawns,
        loaded::{SpawnsSequence, SpawnsSequenceHandle},
    };
    use sprite_loading::SpriteLoadingBundle;
    use sprite_model::loaded::{SpriteRenderSequence, SpriteRenderSequenceHandle};
    use test_object_model::{config::TestObjectSequenceId, loaded::TestObjectObjectWrapper};

    use super::ObjectEntityAugmenter;
    use crate::{FrameComponentStorages, ObjectComponentStorages};

    #[test]
    fn augments_entity_with_object_components() -> Result<(), Error> {
        let assertion = |world: &mut World| {
            let entity = world.create_entity().build();
            {
                let object_wrapper = world.read_resource::<TestObjectObjectWrapper>();

                let mut object_component_storages = ObjectComponentStorages::fetch(&world.res);
                ObjectEntityAugmenter::augment(
                    entity,
                    &mut object_component_storages,
                    &*object_wrapper,
                );
            }

            assert!(world
                .read_storage::<TestObjectSequenceId>()
                .contains(entity));
            assert!(world.read_storage::<SequenceStatus>().contains(entity));
            assert!(world.read_storage::<Mirrored>().contains(entity));
            assert!(world.read_storage::<Transparent>().contains(entity));
            assert!(world.read_storage::<Position<f32>>().contains(entity));
            assert!(world.read_storage::<Velocity<f32>>().contains(entity));
            assert!(world.read_storage::<Transform>().contains(entity));
            assert!(world.read_storage::<FrameIndexClock>().contains(entity));
            assert!(world.read_storage::<FrameWaitClock>().contains(entity));
        };

        AmethystApplication::blank()
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_ui_bundles::<ControlBindings>()
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(CollisionLoadingBundle::new())
            .with_setup(|world| {
                <FrameComponentStorages as SystemData>::setup(&mut world.res);
                <ObjectComponentStorages<TestObjectSequenceId> as SystemData>::setup(
                    &mut world.res,
                );
            })
            .with_setup(setup_object_wrapper)
            .with_assertion(assertion)
            .run_isolated()
    }

    fn setup_object_wrapper(world: &mut World) {
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
            ) = world.system_data::<TestSystemData>();

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
                HashMap::<TestObjectSequenceId, WaitSequenceHandle>::new(),
                HashMap::<TestObjectSequenceId, SpriteRenderSequenceHandle>::new(),
                HashMap::<TestObjectSequenceId, BodySequenceHandle>::new(),
                HashMap::<TestObjectSequenceId, InteractionsSequenceHandle>::new(),
                HashMap::<TestObjectSequenceId, SpawnsSequenceHandle>::new(),
            );
            wait_sequence_handles.insert(TestObjectSequenceId::Zero, wait_sequence_handle);
            sprite_render_sequence_handles
                .insert(TestObjectSequenceId::Zero, sprite_render_sequence_handle);
            body_sequence_handles.insert(TestObjectSequenceId::Zero, body_sequence_handle);
            interactions_sequence_handles
                .insert(TestObjectSequenceId::Zero, interactions_sequence_handle);
            spawns_sequence_handles.insert(TestObjectSequenceId::Zero, spawns_sequence_handle);

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
                .insert(TestObjectSequenceId::Zero, SequenceEndTransition::new(None));
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
        let object_wrapper = TestObjectObjectWrapper(object);

        world.add_resource(object_wrapper);
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

    type TestSystemData<'s> = (
        ObjectLoaderSystemData<'s>,
        Read<'s, AssetStorage<Texture>>,
        Read<'s, AssetStorage<SpriteSheet>>,
    );
}
