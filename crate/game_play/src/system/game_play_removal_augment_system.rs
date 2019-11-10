use amethyst::{
    ecs::{ReadExpect, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_play_model::GamePlayEntity;
use spawn_model::play::SpawnEvent;
use state_registry::StateId;
use typename_derive::TypeName;

/// Augments spawned entities with the `GamePlayEntity` removal component during `GamePlay`.
#[derive(Debug, Default, TypeName, new)]
pub struct GamePlayRemovalAugmentSystem {
    /// Reader ID for the `SpawnEvent` channel.
    #[new(default)]
    spawn_event_rid: Option<ReaderId<SpawnEvent>>,
}

/// `GamePlayRemovalAugmentSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GamePlayRemovalAugmentSystemData<'s> {
    /// `StateId` resource.
    #[derivative(Debug = "ignore")]
    pub state_id: ReadExpect<'s, StateId>,
    /// `SpawnEvent` channel.
    #[derivative(Debug = "ignore")]
    pub spawn_ec: Write<'s, EventChannel<SpawnEvent>>,
    /// `GamePlayEntity` components.
    #[derivative(Debug = "ignore")]
    pub game_play_entities: WriteStorage<'s, GamePlayEntity>,
}

impl<'s> System<'s> for GamePlayRemovalAugmentSystem {
    type SystemData = GamePlayRemovalAugmentSystemData<'s>;

    fn run(
        &mut self,
        GamePlayRemovalAugmentSystemData {
            state_id,
            spawn_ec,
            mut game_play_entities,
        }: Self::SystemData,
    ) {
        let spawn_event_rid = self.spawn_event_rid.as_mut().expect(
            "Expected `spawn_event_rid` field to be set for `GamePlayRemovalAugmentSystem`.",
        );

        // Make sure we don't block the channel from deleting events.
        let spawn_events_iter = spawn_ec.read(spawn_event_rid);

        if *state_id == StateId::GamePlay {
            spawn_events_iter.for_each(|ev| {
                let entity_spawned = ev.entity_spawned;

                game_play_entities
                    .insert(entity_spawned, GamePlayEntity)
                    .expect("Failed to insert `GamePlayEntity` component.");
            });
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.spawn_event_rid = Some(
            world
                .fetch_mut::<EventChannel<SpawnEvent>>()
                .register_reader(),
        );
    }
}
