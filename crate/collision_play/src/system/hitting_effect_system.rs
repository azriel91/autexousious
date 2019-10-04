use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{loaded::HittingTransition, play::HitEvent};
use derivative::Derivative;
use derive_new::new;
use sequence_model::loaded::SequenceId;
use typename_derive::TypeName;

/// Determines the next sequence for entities when they hit another object.
#[derive(Debug, Default, TypeName, new)]
pub struct HittingEffectSystem {
    /// Reader ID for the `HitEvent` event channel.
    #[new(default)]
    hit_event_rid: Option<ReaderId<HitEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct HittingEffectSystemData<'s> {
    /// `HitEvent` channel.
    #[derivative(Debug = "ignore")]
    pub hit_ec: Read<'s, EventChannel<HitEvent>>,
    /// `HittingTransition` components.
    #[derivative(Debug = "ignore")]
    pub hitting_transitions: ReadStorage<'s, HittingTransition>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl<'s> System<'s> for HittingEffectSystem {
    type SystemData = HittingEffectSystemData<'s>;

    fn run(
        &mut self,
        HittingEffectSystemData {
            hit_ec,
            hitting_transitions,
            mut sequence_ids,
        }: Self::SystemData,
    ) {
        hit_ec
            .read(
                self.hit_event_rid
                    .as_mut()
                    .expect("Expected `hit_event_rid` to exist for `HittingEffectSystem`."),
            )
            .for_each(|ev| {
                let hitting_transition = hitting_transitions.get(ev.from).copied();

                if let Some(HittingTransition(sequence_id)) = hitting_transition {
                    sequence_ids
                        .insert(ev.from, sequence_id)
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
