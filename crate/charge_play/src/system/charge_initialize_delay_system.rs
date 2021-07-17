use amethyst::{
    ecs::{Entities, Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use charge_model::{
    config::{ChargeDelay, ChargeLimit},
    play::{ChargeBeginDelayClock, ChargeDelayClock, ChargeStatus, ChargeTrackerClock},
};
use derivative::Derivative;
use derive_new::new;

/// Ticks `ChargeBeginDelayClock` while `Attack` is held.
///
/// Adds `ChargeTrackerClock` on charge begin if none exists and
/// `ChargeBeginDelayClock` `is_complete()`
#[derive(Debug, Default, new)]
pub struct ChargeInitializeDelaySystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChargeInitializeDelaySystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ChargeLimit` components.
    #[derivative(Debug = "ignore")]
    pub charge_limits: ReadStorage<'s, ChargeLimit>,
    /// `ChargeDelay` components.
    #[derivative(Debug = "ignore")]
    pub charge_delays: ReadStorage<'s, ChargeDelay>,
    /// `ChargeStatus` components.
    #[derivative(Debug = "ignore")]
    pub charge_statuses: WriteStorage<'s, ChargeStatus>,
    /// `ChargeBeginDelayClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_begin_delay_clocks: WriteStorage<'s, ChargeBeginDelayClock>,
    /// `ChargeTrackerClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_tracker_clocks: WriteStorage<'s, ChargeTrackerClock>,
    /// `ChargeDelayClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_delay_clocks: WriteStorage<'s, ChargeDelayClock>,
}

impl<'s> System<'s> for ChargeInitializeDelaySystem {
    type SystemData = ChargeInitializeDelaySystemData<'s>;

    fn run(
        &mut self,
        ChargeInitializeDelaySystemData {
            entities,
            charge_limits,
            charge_delays,
            mut charge_statuses,
            mut charge_begin_delay_clocks,
            mut charge_tracker_clocks,
            mut charge_delay_clocks,
        }: Self::SystemData,
    ) {
        (
            &entities,
            charge_limits.maybe(),
            charge_delays.maybe(),
            &mut charge_statuses,
            &mut charge_begin_delay_clocks,
        )
            .join()
            .for_each(
                |(entity, charge_limit, charge_delay, charge_status, charge_begin_delay_clock)| {
                    if *charge_status == ChargeStatus::BeginDelay {
                        charge_begin_delay_clock.tick();

                        if charge_begin_delay_clock.is_complete() {
                            *charge_status = ChargeStatus::Charging;

                            if !charge_tracker_clocks.contains(entity) {
                                let charge_limit =
                                    charge_limit.copied().unwrap_or_else(ChargeLimit::default);
                                let charge_limit = (*charge_limit) as usize;
                                let charge_tracker_clock = ChargeTrackerClock::new(charge_limit);
                                charge_tracker_clocks
                                    .insert(entity, charge_tracker_clock)
                                    .expect("Failed to insert `ChargeTrackerClock` component.");
                            }

                            let charge_delay =
                                charge_delay.copied().unwrap_or_else(ChargeDelay::default);
                            let mut charge_delay_clock = ChargeDelayClock::new(*charge_delay);
                            charge_delay_clock.value = *charge_delay; // Start off as complete.
                            charge_delay_clocks
                                .insert(entity, charge_delay_clock)
                                .expect("Failed to insert `ChargeDelayClock` component.");
                        }
                    }
                },
            );
    }
}
