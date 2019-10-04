use amethyst::{
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use charge_model::play::{ChargeDelayClock, ChargeStatus, ChargeTrackerClock};
use derivative::Derivative;
use derive_new::new;
use typename_derive::TypeName;

/// Ticks `ChargeTrackerClock` while `Charging`.
#[derive(Debug, Default, TypeName, new)]
pub struct ChargeIncrementSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChargeIncrementSystemData<'s> {
    /// `ChargeStatus` components.
    #[derivative(Debug = "ignore")]
    pub charge_statuses: ReadStorage<'s, ChargeStatus>,
    /// `ChargeDelayClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_delay_clocks: WriteStorage<'s, ChargeDelayClock>,
    /// `ChargeTrackerClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_tracker_clocks: WriteStorage<'s, ChargeTrackerClock>,
}

impl<'s> System<'s> for ChargeIncrementSystem {
    type SystemData = ChargeIncrementSystemData<'s>;

    fn run(
        &mut self,
        ChargeIncrementSystemData {
            charge_statuses,
            mut charge_delay_clocks,
            mut charge_tracker_clocks,
        }: Self::SystemData,
    ) {
        (
            &charge_statuses,
            &mut charge_delay_clocks,
            &mut charge_tracker_clocks,
        )
            .join()
            .for_each(
                |(charge_status, charge_delay_clock, charge_tracker_clock)| {
                    if *charge_status == ChargeStatus::Charging {
                        charge_delay_clock.tick();

                        if charge_delay_clock.is_complete() {
                            charge_delay_clock.reset();

                            charge_tracker_clock.tick();
                        }
                    }
                },
            );
    }
}
