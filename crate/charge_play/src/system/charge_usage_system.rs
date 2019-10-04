use amethyst::{
    ecs::{ReadStorage, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use charge_model::{
    config::ChargeUseMode,
    play::{ChargeTrackerClock, ChargeUseEvent},
};
use derivative::Derivative;
use derive_new::new;
use log::warn;
use typename_derive::TypeName;

/// Subtracts `ChargeTrackerClock` when used.
#[derive(Debug, Default, TypeName, new)]
pub struct ChargeUsageSystem {
    /// Reader ID for the `ChargeUseEvent` channel.
    #[new(default)]
    charge_event_rid: Option<ReaderId<ChargeUseEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChargeUsageSystemData<'s> {
    /// `ChargeUseEvent` channel.
    #[derivative(Debug = "ignore")]
    pub charge_ec: Write<'s, EventChannel<ChargeUseEvent>>,
    /// `ChargeUseMode` components.
    #[derivative(Debug = "ignore")]
    pub charge_use_modes: ReadStorage<'s, ChargeUseMode>,
    /// `ChargeTrackerClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_tracker_clocks: WriteStorage<'s, ChargeTrackerClock>,
}

impl<'s> System<'s> for ChargeUsageSystem {
    type SystemData = ChargeUsageSystemData<'s>;

    fn run(
        &mut self,
        ChargeUsageSystemData {
            charge_ec,
            charge_use_modes,
            mut charge_tracker_clocks,
        }: Self::SystemData,
    ) {
        let charge_event_rid = self
            .charge_event_rid
            .as_mut()
            .expect("Expected `charge_event_rid` field to be set.");

        charge_ec.read(charge_event_rid).for_each(|ev| {
            let entity = ev.entity;
            let charge_use = (*ev.charge_points) as usize;

            let charge_tracker_clock = charge_tracker_clocks
                .get_mut(entity)
                .expect("Expected `ChargeTrackerClock` component to exist.");

            let charge_use_mode = charge_use_modes
                .get(entity)
                .copied()
                .unwrap_or_else(|| ChargeUseMode::Exact);

            if charge_tracker_clock.is_beginning() {
                // No charge stored.
                warn!(
                    "Attempted to subtract `{}` charge points from empty charge.",
                    charge_use,
                );

                return;
            }

            let charge_current = (*charge_tracker_clock).value;
            if charge_use > charge_current {
                if charge_use_mode != ChargeUseMode::NearestPartial {
                    warn!(
                        "Attempted to subtract `{}` charge points when `{}` available.",
                        charge_use, charge_current
                    );
                }

                charge_tracker_clock.reset();
                return;
            }

            match charge_use_mode {
                ChargeUseMode::Exact => (*charge_tracker_clock).value = charge_current - charge_use,
                ChargeUseMode::NearestPartial => {
                    let remainder = charge_current % charge_use;
                    if remainder > 0 {
                        (*charge_tracker_clock).value -= remainder;
                    } else {
                        (*charge_tracker_clock).value -= charge_use;
                    }
                }
                ChargeUseMode::NearestWhole => {
                    let remainder = charge_current % charge_use;
                    (*charge_tracker_clock).value -= charge_use + remainder;
                }
                ChargeUseMode::All => charge_tracker_clock.reset(),
            }
        });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.charge_event_rid = Some(
            world
                .fetch_mut::<EventChannel<ChargeUseEvent>>()
                .register_reader(),
        );
    }
}
