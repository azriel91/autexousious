use amethyst::{
    ecs::{ReadExpect, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use state_registry::{StateId, StateIdUpdateEvent};
use tracker::Prev;
use typename_derive::TypeName;

/// Emits `StateIdUpdateEvent`s when the `StateId` changes.
#[derive(Debug, Default, TypeName, new)]
pub struct StateIdEventSystem;

/// `StateIdEventSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StateIdEventSystemData<'s> {
    /// `StateId` resource.
    #[derivative(Debug = "ignore")]
    pub state_id: Option<ReadExpect<'s, StateId>>,
    /// `Prev<StateId>` resource.
    #[derivative(Debug = "ignore")]
    pub state_id_prev: Option<ReadExpect<'s, Prev<StateId>>>,
    /// `StateIdUpdateEvent` channel.
    #[derivative(Debug = "ignore")]
    pub state_id_update_ec: Write<'s, EventChannel<StateIdUpdateEvent>>,
}

impl<'s> System<'s> for StateIdEventSystem {
    type SystemData = StateIdEventSystemData<'s>;

    fn run(
        &mut self,
        StateIdEventSystemData {
            state_id,
            state_id_prev,
            mut state_id_update_ec,
        }: Self::SystemData,
    ) {
        if let Some(state_id) = state_id {
            let state_id = *state_id;
            let state_id_prev = state_id_prev.map(|state_id_prev| **state_id_prev).clone();

            // Send event when `state_id_prev` is `None`, or when it differs from `state_id`.
            if state_id_prev
                .map(|state_id_prev| state_id != state_id_prev)
                .unwrap_or(true)
            {
                let state_id_update_event = StateIdUpdateEvent::new(state_id, state_id_prev);
                state_id_update_ec.single_write(state_id_update_event);
            }
        }
    }
}
