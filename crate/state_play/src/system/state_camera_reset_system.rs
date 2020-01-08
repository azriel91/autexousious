use amethyst::{
    ecs::{Entities, Entity, Join, Read, System, World},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use camera_play::{CameraCreator, CameraCreatorResources};
use derivative::Derivative;
use derive_new::new;
use state_registry::StateIdUpdateEvent;

/// Resets `Camera` transforms when `StateId` changes.
#[derive(Debug, Default, new)]
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
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `CameraCreatorResources`.
    pub camera_creator_resources: CameraCreatorResources<'s>,
}

impl<'s> System<'s> for StateCameraResetSystem {
    type SystemData = StateCameraResetSystemData<'s>;

    fn run(
        &mut self,
        StateCameraResetSystemData {
            state_id_update_ec,
            entities,
            mut camera_creator_resources,
        }: Self::SystemData,
    ) {
        let state_id_update_event_rid = self
            .state_id_update_event_rid
            .as_mut()
            .expect("Expected `state_id_update_event_rid` field to be set.");

        // Make sure events are read.
        let mut events_iterator = state_id_update_ec.read(state_id_update_event_rid);

        if events_iterator.next().is_some() {
            let camera_entities = {
                let cameras = &camera_creator_resources.camera_component_storages.cameras;
                (&entities, cameras)
                    .join()
                    .map(|(entity, _)| entity)
                    .collect::<Vec<Entity>>()
            };

            camera_entities.into_iter().for_each(|entity| {
                CameraCreator::camera_reset(&mut camera_creator_resources, entity);
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
