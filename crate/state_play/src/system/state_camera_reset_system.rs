use amethyst::{
    ecs::{Join, Read, ReadStorage, System, World, WriteStorage},
    renderer::camera::Camera,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;
use state_registry::StateIdUpdateEvent;
use typename_derive::TypeName;

/// Resets `Camera` positions when `StateId` changes.
#[derive(Debug, Default, TypeName, new)]
pub struct StateCameraResetSystem {
    /// Reader ID for the `StateIdUpdateEvent` channel.
    #[new(default)]
    state_id_update_event_rid: Option<ReaderId<StateIdUpdateEvent>>,
}

/// `StateCameraResetSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StateCameraResetSystemData<'s> {
    /// `StateIdUpdateEvent` channel.
    #[derivative(Debug = "ignore")]
    pub state_id_update_ec: Read<'s, EventChannel<StateIdUpdateEvent>>,
    /// `Camera` components.
    #[derivative(Debug = "ignore")]
    pub cameras: ReadStorage<'s, Camera>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
}

impl<'s> System<'s> for StateCameraResetSystem {
    type SystemData = StateCameraResetSystemData<'s>;

    fn run(
        &mut self,
        StateCameraResetSystemData {
            state_id_update_ec,
            cameras,
            mut positions,
        }: Self::SystemData,
    ) {
        let state_id_update_event_rid = self
            .state_id_update_event_rid
            .as_mut()
            .expect("Expected `state_id_update_event_rid` field to be set.");

        // Make sure events are read.
        let mut events_iterator = state_id_update_ec.read(state_id_update_event_rid);
        if events_iterator.next().is_some() {
            (&cameras, &mut positions).join().for_each(|(_, position)| {
                position.x = 0.;
                position.y = 0.;
                position.z = 0.;
            });
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.state_id_update_event_rid = Some(
            world
                .fetch_mut::<EventChannel<StateIdUpdateEvent>>()
                .register_reader(),
        );
    }
}
