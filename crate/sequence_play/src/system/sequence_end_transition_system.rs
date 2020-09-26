use amethyst::{
    ecs::{Entities, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::{
    loaded::{SequenceEndTransition, SequenceId},
    play::SequenceUpdateEvent,
};

/// Transitions an object when their `SequenceUpdateEvent::SequenceEnd`
#[derive(Debug, Default, new)]
pub struct SequenceEndTransitionSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    sequence_update_event_rid: Option<ReaderId<SequenceUpdateEvent>>,
}

/// `SequenceEndTransitionSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceEndTransitionSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `SequenceEndTransition` components.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitions: ReadStorage<'s, SequenceEndTransition>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl<'s> System<'s> for SequenceEndTransitionSystem {
    type SystemData = SequenceEndTransitionSystemData<'s>;

    fn run(
        &mut self,
        SequenceEndTransitionSystemData {
            entities,
            sequence_update_ec,
            sequence_end_transitions,
            mut sequence_ids,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(self.sequence_update_event_rid.as_mut().expect(
                "Expected `sequence_update_event_rid` to exist for \
                 `SequenceEndTransitionSystem`.",
            ))
            .filter(|ev| matches!(ev, SequenceUpdateEvent::SequenceEnd { .. }))
            .for_each(|ev| {
                let entity = ev.entity();

                let sequence_end_transition = sequence_end_transitions.get(entity).copied();

                if let Some(sequence_end_transition) = sequence_end_transition {
                    match sequence_end_transition {
                        SequenceEndTransition::None => {}
                        SequenceEndTransition::Repeat => {
                            let sequence_id = sequence_ids
                                .get(entity)
                                .copied()
                                .expect("Expected entity to have `SequenceId` component.");
                            // Re-insertion causes sequence to restart.
                            sequence_ids
                                .insert(entity, sequence_id)
                                .expect("Failed to insert `SequenceId` component.");
                        }
                        SequenceEndTransition::Delete => {
                            entities
                                .delete(entity)
                                .expect("Failed to delete entity on `SequenceEndTransition`.");
                        }
                        SequenceEndTransition::SequenceId(sequence_id) => {
                            sequence_ids
                                .insert(entity, sequence_id)
                                .expect("Failed to insert `SequenceId` component.");
                        }
                    }
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.sequence_update_event_rid = Some(
            world
                .fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}
