use std::marker::PhantomData;

use amethyst::{
    assets::{AssetStorage, Handle},
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};
use derivative::Derivative;
use derive_new::new;
use logic_clock::LogicClock;
use named_type::NamedType;
use named_type_derive::NamedType;
use object_loading::ObjectFrameComponentStorages;
use object_model::{
    config::object::FrameIndex,
    entity::SequenceStatus,
    loaded::{ComponentSequence, GameObject, ObjectWrapper},
};
use shred_derive::SystemData;

/// Updates the logic clock and sequence ID for objects.
///
/// # Type Parameters
///
/// * `O`: `GameObject` type, e.g. `Character`.
#[derive(Debug, Default, NamedType, new)]
pub(crate) struct ObjectSequenceUpdateSystem<O> {
    /// PhantomData.
    phantom_data: PhantomData<O>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectSequenceUpdateSystemData<'s, O>
where
    O: GameObject,
{
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `Handle<O::ObjectWrapper>` component storage.
    #[derivative(Debug = "ignore")]
    pub object_handles: ReadStorage<'s, Handle<O::ObjectWrapper>>,
    /// `O::ObjectWrapper` assets.
    #[derivative(Debug = "ignore")]
    pub object_assets: Read<'s, AssetStorage<O::ObjectWrapper>>,
    /// `FrameIndex` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_indicies: WriteStorage<'s, FrameIndex>,
    /// `LogicClock` component storage.
    #[derivative(Debug = "ignore")]
    pub logic_clocks: WriteStorage<'s, LogicClock>,
    /// `O::SequenceId` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, O::SequenceId>,
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// Game object `Component` storages.
    pub object_frame_component_storages: ObjectFrameComponentStorages<'s>,
}

impl<'s, O> System<'s> for ObjectSequenceUpdateSystem<O>
where
    O: GameObject,
{
    type SystemData = ObjectSequenceUpdateSystemData<'s, O>;

    fn run(
        &mut self,
        ObjectSequenceUpdateSystemData {
            entities,
            object_handles,
            object_assets,
            mut frame_indicies,
            mut logic_clocks,
            mut sequence_ids,
            mut sequence_statuses,
            object_frame_component_storages,
        }: Self::SystemData,
    ) {
        let ObjectFrameComponentStorages {
            mut waits,
            mut sprite_renders,
            mut bodies,
            mut interactionses,
        } = object_frame_component_storages;

        (
            &entities,
            &object_handles,
            &mut logic_clocks,
            &mut sequence_ids,
            &mut sequence_statuses,
        )
            .join()
            .for_each(
                |(entity, object_handle, logic_clock, sequence_id, sequence_status)| {
                    let object = object_assets
                        .get(object_handle)
                        .expect("Expected object to be loaded.");

                    let component_sequences = match sequence_status {
                        SequenceStatus::Begin => {
                            // Retrieve frame indicies separately as we use a `FlaggedStorage` to
                            // track if it has been changed, to update frame components.
                            let frame_index = frame_indicies
                                .get_mut(entity)
                                .expect("Expected object to have a `FrameIndex`.");
                            **frame_index = 0;
                            logic_clock.reset();

                            // Set to ongoing, meaning we must be sure that this is the only system
                            // that needs to read the `SequenceStatus::Begin` status.
                            *sequence_status = SequenceStatus::Ongoing;

                            let component_sequences = object
                                .inner()
                                .component_sequences
                                .get(sequence_id)
                                .unwrap_or_else(|| {
                                    panic!(
                                        "Failed to get `ComponentSequences` for sequence ID: \
                                         `{:?}`.",
                                        sequence_id
                                    );
                                });

                            Some(component_sequences)
                        }
                        SequenceStatus::Ongoing => {
                            logic_clock.tick();

                            if logic_clock.is_complete() {
                                // Switch to next frame, or if there is no next frame, switch
                                // `SequenceStatus` to `End`.

                                let component_sequences = object
                                    .inner()
                                    .component_sequences
                                    .get(sequence_id)
                                    .unwrap_or_else(|| {
                                        panic!(
                                            "Failed to get `ComponentSequences` for sequence ID: \
                                             `{:?}`.",
                                            sequence_id
                                        );
                                    });

                                let sequence_ended = {
                                    let frame_index = frame_indicies
                                        .get(entity)
                                        .expect("Expected object to have a `FrameIndex`.");

                                    let frame_count = component_sequences.frame_count();
                                    *frame_index == frame_count - 1
                                };

                                if sequence_ended {
                                    *sequence_status = SequenceStatus::End;
                                    None
                                } else {
                                    let frame_index = frame_indicies
                                        .get_mut(entity)
                                        .expect("Expected object to have a `FrameIndex`.");
                                    *frame_index += 1;

                                    logic_clock.reset();

                                    Some(component_sequences)
                                }
                            } else {
                                None
                            }
                        }
                        SequenceStatus::End => None, // do nothing
                    };

                    let frame_index = frame_indicies
                        .get(entity)
                        .expect("Expected object to have a `FrameIndex`.");

                    // TODO: split this into a separate system.
                    if let Some(component_sequences) = component_sequences {
                        component_sequences.iter().for_each(|component_sequence| {
                            match component_sequence {
                                ComponentSequence::Wait(wait_sequence) => {
                                    let wait = wait_sequence[**frame_index];
                                    waits
                                        .insert(entity, wait)
                                        .expect("Failed to insert `Wait` component for object.");

                                    logic_clock.limit = *wait;
                                }
                                ComponentSequence::SpriteRender(sprite_render_sequence) => {
                                    let sprite_render =
                                        sprite_render_sequence[**frame_index].clone();
                                    sprite_renders.insert(entity, sprite_render).expect(
                                        "Failed to insert `SpriteRender` component for object.",
                                    );
                                }
                                ComponentSequence::Body(body_sequence) => {
                                    let body = body_sequence[**frame_index].clone();
                                    bodies
                                        .insert(entity, body)
                                        .expect("Failed to insert `Body` component for object.");
                                }
                                ComponentSequence::Interactions(interactions_sequence) => {
                                    let interactions = interactions_sequence[**frame_index].clone();
                                    interactionses.insert(entity, interactions).expect(
                                        "Failed to insert `Interactions` component for object.",
                                    );
                                }
                            }
                        });
                    }
                },
            );
    }
}
