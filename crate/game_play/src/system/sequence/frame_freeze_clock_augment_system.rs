use amethyst::{
    ecs::{Read, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{
    config::{Interaction, InteractionKind},
    play::HitEvent,
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::play::FrameFreezeClock;

use typename_derive::TypeName;

/// Creates `FrameFreezeClock`s for new `Hit` collisions.
///
/// This attaches `FrameFreezeClock` to the entity with the `Interaction`.
#[derive(Debug, Default, TypeName, new)]
pub struct FrameFreezeClockAugmentSystem {
    /// Reader ID for the `HitEvent` event channel.
    #[new(default)]
    hit_event_rid: Option<ReaderId<HitEvent>>,
}

/// `FrameFreezeClockAugmentSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct FrameFreezeClockAugmentSystemData<'s> {
    /// `HitEvent` channel.
    #[derivative(Debug = "ignore")]
    pub hit_ec: Read<'s, EventChannel<HitEvent>>,
    /// `FrameFreezeClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_freeze_clocks: WriteStorage<'s, FrameFreezeClock>,
}

impl<'s> System<'s> for FrameFreezeClockAugmentSystem {
    type SystemData = FrameFreezeClockAugmentSystemData<'s>;

    fn run(
        &mut self,
        FrameFreezeClockAugmentSystemData {
            hit_ec,
            mut frame_freeze_clocks,
        }: Self::SystemData,
    ) {
        hit_ec
            .read(
                self.hit_event_rid
                    .as_mut()
                    .expect("Expected reader ID to exist for FrameFreezeClockAugmentSystem."),
            )
            .for_each(|ev| {
                // Only add `FrameFreezeClock` for `Hit` interactions.
                let Interaction {
                    kind: InteractionKind::Hit(_),
                    ..
                } = ev.interaction;

                let frame_freeze_clock = FrameFreezeClock::new(3);
                frame_freeze_clocks
                    .insert(ev.from, frame_freeze_clock)
                    .expect("Failed to insert `FrameFreezeClock`.");
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
