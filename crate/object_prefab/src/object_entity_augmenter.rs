use amethyst::{
    assets::AssetStorage, core::transform::Transform, ecs::Entity,
    renderer::transparent::Transparent,
};
use logic_clock::LogicClock;
use object_model::{
    loaded::ObjectWrapper,
    play::{Mirrored, Position, Velocity},
};
use sequence_model::{
    loaded::{ComponentSequence, ComponentSequences},
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};

use crate::{FrameComponentStorages, ObjectComponentStorages};

/// Augments an entity with `Object` components.
#[derive(Debug)]
pub struct ObjectEntityAugmenter;

impl ObjectEntityAugmenter {
    /// Augments an entity with `Object` components.
    ///
    /// # Parameters
    ///
    /// * `entity`: The entity to augment.
    /// * `component_sequences_assets`: Asset storage for `ComponentSequences`.
    /// * `object_component_storages`: Non-frame-dependent `Component` storages for objects.
    /// * `frame_component_storages`: Frame component storages for objects.
    /// * `object_wrapper`: Slug and handle of the object to spawn.
    pub fn augment<'s, W>(
        entity: Entity,
        component_sequences_assets: &AssetStorage<ComponentSequences>,
        ObjectComponentStorages {
            ref mut transparents,
            ref mut positions,
            ref mut velocities,
            ref mut transforms,
            ref mut mirroreds,
            ref mut component_sequences_handles,
            ref mut sequence_end_transitionses,
            ref mut sequence_ids,
            ref mut sequence_statuses,
            ref mut frame_index_clocks,
            ref mut frame_wait_clocks,
        }: &mut ObjectComponentStorages<'s, W::SequenceId>,
        FrameComponentStorages {
            ref mut waits,
            ref mut sprite_renders,
            ref mut bodies,
            ref mut interactionses,
        }: &mut FrameComponentStorages<'s>,
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

        let component_sequences_handle = object_wrapper
            .inner()
            .component_sequences_handles
            .get(&sequence_id)
            .unwrap_or_else(|| {
                // kcov-ignore-start
                panic!(
                    "Failed to get `ComponentSequencesHandle` for sequence ID: \
                     `{:?}`.",
                    sequence_id
                );
                // kcov-ignore-end
            });

        let component_sequences = component_sequences_assets
            .get(component_sequences_handle)
            .unwrap_or_else(|| {
                // kcov-ignore-start
                panic!(
                    "Expected component_sequences to be loaded for sequence_id: `{:?}`",
                    sequence_id
                )
                // kcov-ignore-end
            });

        let frame_index_clock =
            FrameIndexClock::new(LogicClock::new(component_sequences.frame_count()));
        let starting_frame_index = (*frame_index_clock).value;
        frame_index_clocks
            .insert(entity, frame_index_clock)
            .expect("Failed to insert frame_index_clock component.");
        let mut frame_wait_clock = FrameWaitClock::new(LogicClock::new(1));

        component_sequences
            .iter()
            .for_each(|component_sequence| match component_sequence {
                ComponentSequence::Wait(wait_sequence) => {
                    let wait = wait_sequence[starting_frame_index];
                    waits
                        .insert(entity, wait)
                        .expect("Failed to insert `Wait` component for object.");

                    (*frame_wait_clock).limit = *wait as usize;
                }
                ComponentSequence::SpriteRender(sprite_render_sequence) => {
                    let sprite_render = sprite_render_sequence[starting_frame_index].clone();
                    sprite_renders
                        .insert(entity, sprite_render)
                        .expect("Failed to insert `SpriteRender` component for object.");
                }
                ComponentSequence::Body(body_sequence) => {
                    let body_handle = body_sequence[starting_frame_index].clone();
                    bodies
                        .insert(entity, body_handle)
                        .expect("Failed to insert `Body` component for object.");
                }
                ComponentSequence::Interactions(interactions_sequence) => {
                    let interactions_handle = interactions_sequence[starting_frame_index].clone();
                    interactionses
                        .insert(entity, interactions_handle)
                        .expect("Failed to insert `Interactions` component for object.");
                }
            });

        component_sequences_handles
            .insert(entity, component_sequences_handle.clone())
            .expect("Failed to insert component_sequences_handle component.");

        frame_wait_clocks
            .insert(entity, frame_wait_clock)
            .expect("Failed to insert frame_wait_clock component.");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use amethyst::{
        assets::{AssetStorage, Loader, Processor},
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
    use collision_model::config::{Body, Hit, Interaction, InteractionKind, Interactions};
    use fnv::FnvHashMap;
    use game_input_model::ControlBindings;
    use object_model::{
        loaded::Object,
        play::{Mirrored, Position, Velocity},
    };
    use sequence_loading::SequenceLoadingBundle;
    use sequence_model::{
        config::Wait,
        loaded::{
            ComponentSequence, ComponentSequences, ComponentSequencesHandle, SequenceEndTransition,
            SequenceEndTransitions,
        },
        play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
    };
    use sequence_model_spi::loaded::ComponentFrames;
    use shape_model::Volume;
    use sprite_loading::SpriteLoadingBundle;
    use test_object_model::{config::TestObjectSequenceId, loaded::TestObjectObjectWrapper};

    use super::ObjectEntityAugmenter;
    use crate::{FrameComponentStorages, ObjectComponentStorages};

    #[test]
    fn augments_entity_with_object_components() -> Result<(), Error> {
        let assertion = |world: &mut World| {
            let entity = world.create_entity().build();
            {
                let object_wrapper = world.read_resource::<TestObjectObjectWrapper>();

                let component_sequences_assets =
                    world.read_resource::<AssetStorage<ComponentSequences>>();
                let mut object_component_storages = ObjectComponentStorages::fetch(&world.res);
                let mut frame_component_storages = FrameComponentStorages::fetch(&world.res);
                ObjectEntityAugmenter::augment(
                    entity,
                    &component_sequences_assets,
                    &mut object_component_storages,
                    &mut frame_component_storages,
                    &*object_wrapper,
                );
            }

            assert!(world
                .read_storage::<TestObjectSequenceId>()
                .contains(entity));
            assert!(world.read_storage::<SequenceStatus>().contains(entity));
            assert!(world.read_storage::<Mirrored>().contains(entity));
            assert!(world.read_storage::<SpriteRender>().contains(entity));
            assert!(world.read_storage::<Transparent>().contains(entity));
            assert!(world.read_storage::<Position<f32>>().contains(entity));
            assert!(world.read_storage::<Velocity<f32>>().contains(entity));
            assert!(world.read_storage::<Transform>().contains(entity));
            assert!(world
                .read_storage::<ComponentSequencesHandle>()
                .contains(entity));
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
        let component_sequences_handles = {
            let loader = world.read_resource::<Loader>();
            let wait_sequence = ComponentFrames::new(vec![Wait::new(2)]);

            let texture_assets = world.read_resource::<AssetStorage<Texture>>();
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
            let sprite_sheet_assets = world.system_data::<Read<'_, AssetStorage<SpriteSheet>>>();
            let sprite_sheet_handle = loader.load_from_data(sprite_sheet, (), &sprite_sheet_assets);
            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle,
                sprite_number: 0,
            };
            let sprite_render_sequence = ComponentFrames::new(vec![sprite_render]);

            let body_assets = world.system_data::<Read<'_, AssetStorage<Body>>>();
            let body_handle = loader.load_from_data(body(), (), &body_assets);
            let body_sequence = ComponentFrames::new(vec![body_handle]);

            let interactions_assets = world.system_data::<Read<'_, AssetStorage<Interactions>>>();
            let interactions_handle =
                loader.load_from_data(interactions(), (), &interactions_assets);
            let interactions_sequence = ComponentFrames::new(vec![interactions_handle]);

            let wait_sequence = ComponentSequence::Wait(wait_sequence);
            let sprite_render_sequence = ComponentSequence::SpriteRender(sprite_render_sequence);
            let body_sequence = ComponentSequence::Body(body_sequence);
            let interactions_sequence = ComponentSequence::Interactions(interactions_sequence);

            let component_sequences = vec![
                wait_sequence,
                sprite_render_sequence,
                body_sequence,
                interactions_sequence,
            ];
            let component_sequences = ComponentSequences::new(component_sequences);
            let component_sequences_assets =
                world.read_resource::<AssetStorage<ComponentSequences>>();
            let component_sequences_handle =
                loader.load_from_data(component_sequences, (), &component_sequences_assets);

            let mut component_sequences_handles = HashMap::new();
            component_sequences_handles
                .insert(TestObjectSequenceId::Zero, component_sequences_handle);
            component_sequences_handles
        };
        let sequence_end_transitions = {
            let mut sequence_end_transitions = FnvHashMap::default();
            sequence_end_transitions
                .insert(TestObjectSequenceId::Zero, SequenceEndTransition::new(None));
            SequenceEndTransitions(sequence_end_transitions)
        };
        let object = Object::new(component_sequences_handles, sequence_end_transitions);
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
}
