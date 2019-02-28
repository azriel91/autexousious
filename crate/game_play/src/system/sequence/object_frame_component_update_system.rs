use object_model::loaded::ComponentSequences;
use std::marker::PhantomData;

use amethyst::{
    assets::{AssetStorage, Handle},
    ecs::{Read, ReadStorage, System, SystemData, WriteStorage},
    shred::Resources,
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use logic_clock::LogicClock;
use named_type::NamedType;
use named_type_derive::NamedType;
use object_loading::ObjectFrameComponentStorages;
use object_model::{
    entity::FrameIndexClock,
    loaded::{ComponentSequence, GameObject, ObjectWrapper},
};
use shred_derive::SystemData;

use crate::ObjectSequenceUpdateEvent;

/// Updates the logic clock and sequence ID for objects.
///
/// # Type Parameters
///
/// * `O`: `GameObject` type, e.g. `Character`.
#[derive(Debug, Default, NamedType, new)]
pub struct ObjectFrameComponentUpdateSystem<O> {
    /// Reader ID for the `ObjectSequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<ObjectSequenceUpdateEvent>>,
    /// PhantomData.
    phantom_data: PhantomData<O>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectFrameComponentUpdateSystemData<'s, O>
where
    O: GameObject,
{
    /// Event channel for `ObjectSequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub object_sequence_update_ec: Read<'s, EventChannel<ObjectSequenceUpdateEvent>>,
    /// `Handle<O::ObjectWrapper>` component storage.
    #[derivative(Debug = "ignore")]
    pub object_handles: ReadStorage<'s, Handle<O::ObjectWrapper>>,
    /// `O::ObjectWrapper` assets.
    #[derivative(Debug = "ignore")]
    pub object_assets: Read<'s, AssetStorage<O::ObjectWrapper>>,
    /// `FrameIndexClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: ReadStorage<'s, FrameIndexClock>,
    /// `LogicClock` component storage.
    #[derivative(Debug = "ignore")]
    pub logic_clocks: WriteStorage<'s, LogicClock>,
    /// `O::SequenceId` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, O::SequenceId>,
    /// Game object `Component` storages.
    pub object_frame_component_storages: ObjectFrameComponentStorages<'s>,
}

impl<O> ObjectFrameComponentUpdateSystem<O>
where
    O: GameObject,
{
    fn component_sequences<'res>(
        object_assets: &'res AssetStorage<O::ObjectWrapper>,
        object_handle: &Handle<O::ObjectWrapper>,
        sequence_id: O::SequenceId,
    ) -> &'res ComponentSequences {
        let object = object_assets
            .get(object_handle)
            .expect("Expected object to be loaded.");
        object
            .inner()
            .component_sequences
            .get(&sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to get `ComponentSequences` for sequence ID: \
                     `{:?}`.",
                    sequence_id
                );
            })
    }
}

impl<'s, O> System<'s> for ObjectFrameComponentUpdateSystem<O>
where
    O: GameObject,
{
    type SystemData = ObjectFrameComponentUpdateSystemData<'s, O>;

    fn run(
        &mut self,
        ObjectFrameComponentUpdateSystemData {
            object_sequence_update_ec,
            object_handles,
            object_assets,
            frame_index_clocks,
            mut logic_clocks,
            sequence_ids,
            object_frame_component_storages,
        }: Self::SystemData,
    ) {
        let ObjectFrameComponentStorages {
            mut waits,
            mut sprite_renders,
            mut bodies,
            mut interactionses,
        } = object_frame_component_storages;

        object_sequence_update_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for ObjectFrameComponentUpdateSystem."),
            )
            .for_each(|ev| match ev {
                ObjectSequenceUpdateEvent::SequenceBegin { entity }
                | ObjectSequenceUpdateEvent::FrameBegin { entity } => {
                    let object_handle = object_handles
                        .get(*entity)
                        .expect("Expected entity to have an `ObjectHandle` component.");
                    let frame_index_clock = frame_index_clocks
                        .get(*entity)
                        .expect("Expected entity to have a `FrameIndexClock` component.");
                    let logic_clock = logic_clocks
                        .get_mut(*entity)
                        .expect("Expected entity to have a `LogicClock` component.");
                    let sequence_id = sequence_ids
                        .get(*entity)
                        .expect("Expected entity to have a `SequenceId` component.");

                    let component_sequences =
                        Self::component_sequences(&object_assets, &object_handle, *sequence_id);

                    let frame_index = (*frame_index_clock).value;

                    component_sequences.iter().for_each(|component_sequence| {
                        match component_sequence {
                            ComponentSequence::Wait(wait_sequence) => {
                                let wait = wait_sequence[frame_index];
                                waits
                                    .insert(*entity, wait)
                                    .expect("Failed to insert `Wait` component for object.");

                                logic_clock.limit = *wait as usize;
                            }
                            ComponentSequence::SpriteRender(sprite_render_sequence) => {
                                let sprite_render = sprite_render_sequence[frame_index].clone();
                                sprite_renders.insert(*entity, sprite_render).expect(
                                    "Failed to insert `SpriteRender` component for object.",
                                );
                            }
                            ComponentSequence::Body(body_sequence) => {
                                let body = body_sequence[frame_index].clone();
                                bodies
                                    .insert(*entity, body)
                                    .expect("Failed to insert `Body` component for object.");
                            }
                            ComponentSequence::Interactions(interactions_sequence) => {
                                let interactions = interactions_sequence[frame_index].clone();
                                interactionses.insert(*entity, interactions).expect(
                                    "Failed to insert `Interactions` component for object.",
                                );
                            }
                        }
                    });
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(
            res.fetch_mut::<EventChannel<ObjectSequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}
