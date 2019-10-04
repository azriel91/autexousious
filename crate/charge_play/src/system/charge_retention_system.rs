use amethyst::{
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use charge_model::play::{ChargeRetention, ChargeStatus, ChargeTrackerClock};
use derivative::Derivative;
use derive_new::new;
use typename_derive::TypeName;

/// Reduces charge when not charging.
#[derive(Debug, Default, TypeName, new)]
pub struct ChargeRetentionSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChargeRetentionSystemData<'s> {
    /// `ChargeStatus` components.
    #[derivative(Debug = "ignore")]
    pub charge_statuses: ReadStorage<'s, ChargeStatus>,
    /// `ChargeRetention` components.
    #[derivative(Debug = "ignore")]
    pub charge_retentions: WriteStorage<'s, ChargeRetention>,
    /// `ChargeTrackerClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_tracker_clocks: WriteStorage<'s, ChargeTrackerClock>,
}

impl<'s> System<'s> for ChargeRetentionSystem {
    type SystemData = ChargeRetentionSystemData<'s>;

    fn run(
        &mut self,
        ChargeRetentionSystemData {
            charge_statuses,
            mut charge_retentions,
            mut charge_tracker_clocks,
        }: Self::SystemData,
    ) {
        (
            &charge_statuses,
            &mut charge_retentions,
            &mut charge_tracker_clocks,
        )
            .join()
            .for_each(|(charge_status, charge_retention, charge_tracker_clock)| {
                if *charge_status == ChargeStatus::NotCharging {
                    match charge_retention {
                        ChargeRetention::Forever => {}
                        ChargeRetention::Never => charge_tracker_clock.reset(),
                        ChargeRetention::Lossy(charge_retention_clock) => {
                            charge_retention_clock.tick();
                            if charge_retention_clock.is_complete() {
                                charge_retention_clock.reset();
                                charge_tracker_clock.reverse_tick();
                            }
                        }
                        ChargeRetention::Reset(charge_retention_clock) => {
                            charge_retention_clock.tick();
                            if charge_retention_clock.is_complete() {
                                charge_retention_clock.reset();
                                charge_tracker_clock.reset();
                            }
                        }
                    }
                } else {
                    match charge_retention {
                        ChargeRetention::Lossy(charge_retention_clock)
                        | ChargeRetention::Reset(charge_retention_clock) => {
                            charge_retention_clock.reset()
                        }
                        _ => {}
                    }
                }
            });
    }
}
