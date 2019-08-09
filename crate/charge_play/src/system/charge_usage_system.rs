use amethyst::{
    ecs::{System, SystemData, Write, WriteStorage},
    shred::Resources,
    shrev::{EventChannel, ReaderId},
};
use charge_model::play::{ChargeTrackerClock, ChargeUseEvent};
use derivative::Derivative;
use derive_new::new;
use shred_derive::SystemData;
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
            mut charge_tracker_clocks,
        }: Self::SystemData,
    ) {
        let charge_event_rid = self
            .charge_event_rid
            .as_mut()
            .expect("Expected `charge_event_rid` field to be set.");

        charge_ec.read(charge_event_rid).for_each(|ev| {
            let charge_tracker_clock = charge_tracker_clocks
                .get_mut(ev.entity)
                .expect("Expected `ChargeTrackerClock` component to exist.");

            (*charge_tracker_clock).value -= (*ev.charge_points) as usize;
        });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.charge_event_rid = Some(
            res.fetch_mut::<EventChannel<ChargeUseEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, ReadStorage, World},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::{
        config::ChargePoints,
        play::{ChargeTrackerClock, ChargeUseEvent},
    };

    use super::ChargeUsageSystem;

    #[test]
    fn subtracts_charge_points_exact() -> Result<(), Error> {
        let mut charge_tracker_clock = ChargeTrackerClock::new(10);
        (*charge_tracker_clock).value = 10;
        let charge_use_event_fn = |entity| ChargeUseEvent::new(entity, ChargePoints::new(3));

        run_test(
            SetupParams {
                charge_tracker_clock,
                charge_use_event_fn,
            },
            ChargePoints::new(7),
        )
    }

    fn run_test(
        SetupParams {
            charge_tracker_clock: charge_tracker_clock_setup,
            charge_use_event_fn,
        }: SetupParams,
        charge_points_expected: ChargePoints,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ChargeUsageSystem::new(), "", &[])
            .with_setup(move |world| {
                let entity = world
                    .create_entity()
                    .with(charge_tracker_clock_setup)
                    .build();
                let charge_use_event = charge_use_event_fn(entity);

                send_event(world, charge_use_event);

                world.add_resource(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let charge_tracker_clocks =
                    world.system_data::<ReadStorage<'_, ChargeTrackerClock>>();

                let charge_tracker_clock = charge_tracker_clocks
                    .get(entity)
                    .copied()
                    .expect("Expected `ChargeTrackerClock` component to exist.");
                assert_eq!(
                    (*charge_points_expected) as usize,
                    (*charge_tracker_clock).value
                );
            })
            .run()
    }

    fn send_event(world: &mut World, charge_use_event: ChargeUseEvent) {
        let mut charge_use_ec = world.write_resource::<EventChannel<ChargeUseEvent>>();
        charge_use_ec.single_write(charge_use_event);
    }

    struct SetupParams {
        charge_tracker_clock: ChargeTrackerClock,
        charge_use_event_fn: fn(Entity) -> ChargeUseEvent,
    }
}
