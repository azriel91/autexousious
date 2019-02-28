use std::marker::PhantomData;

use amethyst::{
    assets::{AssetStorage, Handle},
    ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use logic_clock::LogicClock;
use named_type::NamedType;
use named_type_derive::NamedType;
use object_model::{
    entity::{FrameIndexClock, SequenceStatus},
    loaded::{GameObject, ObjectWrapper},
};
use shred_derive::SystemData;

use crate::ObjectSequenceUpdateEvent;

/// Updates the logic clock and sequence ID for objects.
///
/// # Type Parameters
///
/// * `O`: `GameObject` type, e.g. `Character`.
#[derive(Debug, Default, NamedType, new)]
pub struct ObjectSequenceUpdateSystem<O> {
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
    /// `FrameIndexClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
    /// `LogicClock` component storage.
    #[derivative(Debug = "ignore")]
    pub logic_clocks: WriteStorage<'s, LogicClock>,
    /// `O::SequenceId` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, O::SequenceId>,
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// Event channel for `ObjectSequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub object_sequence_update_ec: Write<'s, EventChannel<ObjectSequenceUpdateEvent>>,
}

impl<O> ObjectSequenceUpdateSystem<O>
where
    O: GameObject,
{
    fn sequence_frame_count(
        object_assets: &AssetStorage<O::ObjectWrapper>,
        object_handle: &Handle<O::ObjectWrapper>,
        sequence_id: O::SequenceId,
    ) -> usize {
        let object = object_assets
            .get(object_handle)
            .expect("Expected object to be loaded.");
        let component_sequences = object
            .inner()
            .component_sequences
            .get(&sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to get `ComponentSequences` for sequence ID: \
                     `{:?}`.",
                    sequence_id
                );
            });

        component_sequences.frame_count()
    }
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
            mut frame_index_clocks,
            mut logic_clocks,
            mut sequence_ids,
            mut sequence_statuses,
            mut object_sequence_update_ec,
        }: Self::SystemData,
    ) {
        (
            &entities,
            &object_handles,
            &mut frame_index_clocks,
            &mut logic_clocks,
            &mut sequence_ids,
            &mut sequence_statuses,
        )
            .join()
            .for_each(
                |(
                    entity,
                    object_handle,
                    frame_index_clock,
                    logic_clock,
                    sequence_id,
                    sequence_status,
                )| {
                    match sequence_status {
                        SequenceStatus::Begin => {
                            // Retrieve frame indicies separately as we use a `FlaggedStorage` to
                            // track if it has been changed, to update frame components.
                            frame_index_clock.reset();
                            logic_clock.reset();

                            // Set to ongoing, meaning we must be sure that this is the only system
                            // that needs to read the `SequenceStatus::Begin` status.
                            *sequence_status = SequenceStatus::Ongoing;

                            // Update the frame_index_clock limit because we already hold a mutable
                            // borrow of the component storage.
                            (*frame_index_clock).limit = Self::sequence_frame_count(
                                &object_assets,
                                &object_handle,
                                *sequence_id,
                            );

                            object_sequence_update_ec
                                .single_write(ObjectSequenceUpdateEvent::SequenceBegin { entity });
                        }
                        SequenceStatus::Ongoing => {
                            logic_clock.tick();

                            if logic_clock.is_complete() {
                                // Switch to next frame, or if there is no next frame, switch
                                // `SequenceStatus` to `End`.

                                logic_clock.reset();
                                frame_index_clock.tick();

                                if frame_index_clock.is_complete() {
                                    *sequence_status = SequenceStatus::End;
                                } else {
                                    object_sequence_update_ec.single_write(
                                        ObjectSequenceUpdateEvent::FrameBegin { entity },
                                    );
                                }
                            }
                        }
                        SequenceStatus::End => {} // do nothing
                    }
                },
            );
    }
}
