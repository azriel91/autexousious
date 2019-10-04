use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{loaded::HitTransition, play::HitEvent};
use derivative::Derivative;
use derive_new::new;
use sequence_model::loaded::SequenceId;
use typename_derive::TypeName;

/// Determines the next sequence for entities when they are hit.
#[derive(Debug, Default, TypeName, new)]
pub struct HitEffectSystem {
    /// Reader ID for the `HitEvent` event channel.
    #[new(default)]
    hit_event_rid: Option<ReaderId<HitEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct HitEffectSystemData<'s> {
    /// `HitEvent` channel.
    #[derivative(Debug = "ignore")]
    pub hit_ec: Read<'s, EventChannel<HitEvent>>,
    /// `HitTransition` components.
    #[derivative(Debug = "ignore")]
    pub hit_transitions: ReadStorage<'s, HitTransition>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl<'s> System<'s> for HitEffectSystem {
    type SystemData = HitEffectSystemData<'s>;

    fn run(
        &mut self,
        HitEffectSystemData {
            hit_ec,
            hit_transitions,
            mut sequence_ids,
        }: Self::SystemData,
    ) {
        hit_ec
            .read(
                self.hit_event_rid
                    .as_mut()
                    .expect("Expected `hit_event_rid` to exist for `HitEffectSystem`."),
            )
            .for_each(|ev| {
                let hit_transition = hit_transitions.get(ev.to).copied();

                if let Some(HitTransition(sequence_id)) = hit_transition {
                    sequence_ids
                        .insert(ev.to, sequence_id)
                        .expect("Failed to insert `SequenceId` component.");
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.hit_event_rid = Some(
            world
                .fetch_mut::<EventChannel<HitEvent>>()
                .register_reader(),
        );
    }
}
