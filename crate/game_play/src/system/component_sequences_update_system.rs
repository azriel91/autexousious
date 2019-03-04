use std::marker::PhantomData;

use amethyst::{
    assets::{AssetStorage, Handle},
    ecs::{
        storage::{ComponentEvent, Tracked},
        BitSet, Component, Join, Read, ReadStorage, ReaderId, System, SystemData, WriteStorage,
    },
    shred::Resources,
};
use derivative::Derivative;
use derive_new::new;
use log::error;
use named_type::NamedType;
use named_type_derive::NamedType;
use object_model::loaded::{GameObject, ObjectWrapper};
use sequence_model::loaded::ComponentSequencesHandle;
use shred_derive::SystemData;

/// Updates the attached `ComponentSequencesHandle`s when the `SequenceId` is changed.
#[derive(Debug, Default, NamedType, new)]
pub struct ComponentSequencesUpdateSystem<O>
where
    O: GameObject,
{
    /// Reader ID for sequence ID changes.
    #[new(default)]
    sequence_id_rid: Option<ReaderId<ComponentEvent>>,
    /// Pre-allocated bitset to track insertions and modifications to `O::SequenceId`s.
    #[new(default)]
    sequence_id_updates: BitSet,
    /// Marker.
    phantom_data: PhantomData<O>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ComponentSequencesUpdateSystemData<'s, O>
where
    O: GameObject,
{
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: ReadStorage<'s, O::SequenceId>,
    /// `Handle<O::ObjectWrapper>` component storage.
    #[derivative(Debug = "ignore")]
    pub object_wrapper_handles: ReadStorage<'s, Handle<O::ObjectWrapper>>,
    /// `O::ObjectWrapper` assets.
    #[derivative(Debug = "ignore")]
    pub object_wrapper_assets: Read<'s, AssetStorage<O::ObjectWrapper>>,
    /// `ComponentSequencesHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub component_sequences_handles: WriteStorage<'s, ComponentSequencesHandle>,
}

impl<'s, O> System<'s> for ComponentSequencesUpdateSystem<O>
where
    O: GameObject,
    <O::SequenceId as Component>::Storage: Tracked,
{
    type SystemData = ComponentSequencesUpdateSystemData<'s, O>;

    fn run(
        &mut self,
        ComponentSequencesUpdateSystemData {
            sequence_ids,
            object_wrapper_handles,
            object_wrapper_assets,
            mut component_sequences_handles,
        }: Self::SystemData,
    ) {
        self.sequence_id_updates.clear();

        sequence_ids
            .channel()
            .read(
                self.sequence_id_rid
                    .as_mut()
                    .expect("Expected `sequence_id_rid` to be set."),
            )
            .for_each(|event| match event {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    self.sequence_id_updates.add(*id);
                }
                ComponentEvent::Removed(_id) => {}
            });

        (&sequence_ids, &object_wrapper_handles, &mut component_sequences_handles, &self.sequence_id_updates)
            .join()
            .for_each(|(sequence_id, object_wrapper_handle, component_sequences_handle, _)| {
                let object_wrapper = object_wrapper_assets.get(&object_wrapper_handle).expect("Expected `ObjectWrapper` to be loaded.");
                let component_sequences_handles = &object_wrapper
                    .inner()
                    .component_sequences_handles;

                *component_sequences_handle = component_sequences_handles
                    .get(&sequence_id)
                    .unwrap_or_else(|| {
                        let message = format!("Expected component sequences to exist for sequence ID: `{:?}`.\
                            Falling back to default sequence component sequences.", sequence_id);
                        error!("{}", message);

                        let default_sequence_id = O::SequenceId::default();

                        component_sequences_handles
                            .get(&default_sequence_id)
                            .unwrap_or_else(|| {
                                let message = format!("Failed to get component sequences for sequence ID: `{:?}`.", default_sequence_id);
                                error!("{}", message);
                                panic!(message);
                            })
                    })
                    .clone()
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.sequence_id_rid = Some(WriteStorage::<O::SequenceId>::fetch(&res).register_reader());
    }
}