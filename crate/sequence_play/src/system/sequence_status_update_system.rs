use amethyst::{
    ecs::{
        storage::ComponentEvent, BitSet, Entities, Join, ReadStorage, ReaderId, System, World,
        Write, WriteStorage,
    },
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::{
    loaded::SequenceId,
    play::{SequenceStatus, SequenceUpdateEvent},
};
use typename_derive::TypeName;

/// Updates `SequenceStatus` to `Begin` when `SequenceId` changes, and sends `SequenceBegin` events.
///
/// This **must** run before `SequenceUpdateSystem`, as that relies on the `SequenceStatus` to
/// determine if a `SequenceBegin` event should be sent.
#[derive(Debug, Default, TypeName, new)]
pub struct SequenceStatusUpdateSystem {
    /// Reader ID for sequence ID changes.
    #[new(default)]
    sequence_id_rid: Option<ReaderId<ComponentEvent>>,
    /// Pre-allocated bitset to track insertions and modifications to `SequenceId`s.
    #[new(default)]
    sequence_id_updates: BitSet,
}

/// `SequenceStatusUpdateSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceStatusUpdateSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: ReadStorage<'s, SequenceId>,
    /// `SequenceStatus` components.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Write<'s, EventChannel<SequenceUpdateEvent>>,
}

impl<'s> System<'s> for SequenceStatusUpdateSystem {
    type SystemData = SequenceStatusUpdateSystemData<'s>;

    fn run(
        &mut self,
        SequenceStatusUpdateSystemData {
            entities,
            sequence_ids,
            mut sequence_statuses,
            mut sequence_update_ec,
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

        (&entities, &sequence_ids, &self.sequence_id_updates)
            .join()
            .for_each(|(entity, sequence_id, _)| {
                sequence_statuses
                    .insert(entity, SequenceStatus::Begin)
                    .expect("Failed to insert `SequenceStatus` component.");

                sequence_update_ec.single_write(SequenceUpdateEvent::SequenceBegin {
                    entity,
                    sequence_id: *sequence_id,
                });
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.sequence_id_rid = Some(WriteStorage::<'_, SequenceId>::fetch(world).register_reader());
    }
}
