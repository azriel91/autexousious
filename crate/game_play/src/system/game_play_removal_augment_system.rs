use amethyst::{
    ecs::{ReadExpect, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_play_model::{GamePlayEntity, GamePlayEntityId};
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
                    .insert(
                        entity_spawned,
                        GamePlayEntity::new(GamePlayEntityId::default()),
                    )
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use amethyst::{
        ecs::{Builder, Entity, Read, SystemData, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{config::AssetSlug, loaded::AssetIdMappings};
    use game_play_model::GamePlayEntity;
    use kinematic_model::config::{Position, Velocity};
    use sequence_model::loaded::SequenceId;
    use spawn_model::{loaded::Spawn, play::SpawnEvent};
    use state_registry::StateId;
    use typename::TypeName;

    use super::GamePlayRemovalAugmentSystem;

    #[test]
    fn augments_removal_when_state_id_is_game_play() -> Result<(), Error> {
        run_test(StateId::GamePlay, true)
    }

    #[test]
    fn does_not_augment_removal_when_state_id_is_not_game_play() -> Result<(), Error> {
        run_test(StateId::MapSelection, false)
    }

    fn run_test(state_id: StateId, has_removal_expected: bool) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(setup_system_data)
            .with_system(
                GamePlayRemovalAugmentSystem::new(),
                GamePlayRemovalAugmentSystem::type_name(),
                &[],
            )
            .with_resource(state_id)
            .with_effect(spawn_entity)
            .with_assertion(move |world| assert_has_removal(world, has_removal_expected))
            .run()
    }

    fn spawn_entity(world: &mut World) {
        let entity_parent = world.create_entity().build();
        let entity_spawned = world.create_entity().build();
        world.insert(entity_spawned);

        let asset_slug = AssetSlug::from_str("default/fireball")
            .expect("Expected `default/fireball` to be a valid asset slug.");
        let asset_id = {
            let mut asset_id_mappings = world.write_resource::<AssetIdMappings>();
            asset_id_mappings.insert(asset_slug)
        };

        let spawn = Spawn::new(
            asset_id,
            Position::<f32>::new(10., 20., 30.),
            Velocity::<f32>::new(40., 50., 60.),
            SequenceId::new(0),
        );

        send_event(
            world,
            SpawnEvent::new(spawn, entity_parent, entity_spawned, asset_id),
        );
    }

    fn send_event(world: &mut World, spawn_event: SpawnEvent) {
        let mut ec = world.write_resource::<EventChannel<SpawnEvent>>();
        ec.single_write(spawn_event);
    } // kcov-ignore

    fn assert_has_removal(world: &mut World, has_removal: bool) {
        let entity_spawned = *world.read_resource::<Entity>();
        let game_play_entities = world.read_storage::<GamePlayEntity>();
        let game_play_entity_actual = game_play_entities.get(entity_spawned);

        assert_eq!(has_removal, game_play_entity_actual.is_some());
    }

    fn setup_system_data(world: &mut World) {
        <Read<'_, AssetIdMappings> as SystemData>::setup(world);
    }
}
