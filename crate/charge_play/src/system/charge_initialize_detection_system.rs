use amethyst::{
    ecs::{Entity, Read, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use charge_model::play::{ChargeBeginDelayClock, ChargeStatus};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{ControlAction, ControlActionEventData, ControlInputEvent};

/// Default number of ticks to wait before beginning to charge.
pub const CHARGE_DELAY_DEFAULT: usize = 10;

/// Detects the begin / cancellation of the initialization phase of charging.
///
/// Resets `ChargeBeginDelayClock` upon charge start / stop (currently control input event release).
#[derive(Debug, Default, new)]
pub struct ChargeInitializeDetectionSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChargeInitializeDetectionSystemData<'s> {
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `ChargeStatus` components.
    #[derivative(Debug = "ignore")]
    pub charge_statuses: WriteStorage<'s, ChargeStatus>,
    /// `ChargeBeginDelayClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_begin_delay_clocks: WriteStorage<'s, ChargeBeginDelayClock>,
}

impl ChargeInitializeDetectionSystem {
    fn update_charge_components(
        charge_statuses: &mut WriteStorage<'_, ChargeStatus>,
        charge_begin_delay_clocks: &mut WriteStorage<'_, ChargeBeginDelayClock>,
        entity: Entity,
        charge_status: ChargeStatus,
    ) {
        charge_statuses
            .insert(entity, charge_status)
            .expect("Failed to insert `ChargeStatus` component.");
        charge_begin_delay_clocks
            .insert(entity, ChargeBeginDelayClock::new(CHARGE_DELAY_DEFAULT))
            .expect("Failed to insert `ChargeBeginDelayClock` component.");
    }
}

impl<'s> System<'s> for ChargeInitializeDetectionSystem {
    type SystemData = ChargeInitializeDetectionSystemData<'s>;

    fn run(
        &mut self,
        ChargeInitializeDetectionSystemData {
            control_input_ec,
            mut charge_statuses,
            mut charge_begin_delay_clocks,
        }: Self::SystemData,
    ) {
        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");

        control_input_ec
            .read(control_input_event_rid)
            .for_each(|ev| match ev {
                ControlInputEvent::ControlActionPress(ControlActionEventData {
                    entity,
                    control_action,
                    ..
                }) => {
                    if *control_action == ControlAction::Attack {
                        Self::update_charge_components(
                            &mut charge_statuses,
                            &mut charge_begin_delay_clocks,
                            *entity,
                            ChargeStatus::BeginDelay,
                        );
                    }
                }
                ControlInputEvent::ControlActionRelease(ControlActionEventData {
                    entity,
                    control_action,
                    ..
                }) => {
                    if *control_action == ControlAction::Attack {
                        Self::update_charge_components(
                            &mut charge_statuses,
                            &mut charge_begin_delay_clocks,
                            *entity,
                            ChargeStatus::NotCharging,
                        );
                    }
                }
                _ => {}
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.control_input_event_rid = Some(
            world
                .fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}
