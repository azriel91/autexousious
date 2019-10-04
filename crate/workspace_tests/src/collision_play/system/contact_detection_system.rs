#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::{
        config::{Hit, HitLimit, HitRepeatDelay, Interaction, InteractionKind},
        play::{CollisionEvent, ContactEvent},
    };
    use pretty_assertions::assert_eq;
    use shape_model::Volume;
    use spawn_model::play::SpawnParent;
    use team_model::play::{IndependentCounter, Team};

    use collision_play::ContactDetectionSystem;

    const HIT_LIMIT: u32 = 3;

    #[test]
    fn inserts_contact_event_when_neither_entity_is_spawn_parent() -> Result<(), Error> {
        run_test(
            SpawnParentVariant::NoSpawnParent,
            TeamsVariant::NoTeam,
            |entity_from, entity_to| vec![contact_event(entity_from, entity_to)],
        )
    }

    #[test]
    fn does_not_insert_contact_event_when_hitter_entity_is_spawn_parent() -> Result<(), Error> {
        run_test(
            SpawnParentVariant::HitterEntityIsSpawnParent,
            TeamsVariant::NoTeam,
            |_, _| vec![],
        )
    }

    #[test]
    fn does_not_insert_contact_event_when_hit_entity_is_spawn_parent() -> Result<(), Error> {
        run_test(
            SpawnParentVariant::HitEntityIsSpawnParent,
            TeamsVariant::NoTeam,
            |_, _| vec![],
        )
    }

    #[test]
    fn inserts_contact_event_when_entities_on_different_teams() -> Result<(), Error> {
        run_test(
            SpawnParentVariant::NoSpawnParent,
            TeamsVariant::DifferentTeam,
            |entity_from, entity_to| vec![contact_event(entity_from, entity_to)],
        )
    }

    #[test]
    fn does_not_insert_contact_event_when_entities_on_same_team() -> Result<(), Error> {
        run_test(
            SpawnParentVariant::NoSpawnParent,
            TeamsVariant::SameTeam,
            |_, _| vec![],
        )
    }

    fn run_test(
        spawn_parent_variant: SpawnParentVariant,
        teams_variant: TeamsVariant,
        events_expected_fn: fn(Entity, Entity) -> Vec<ContactEvent>,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ContactDetectionSystem::new(), "", &[])
            .with_effect(setup_event_reader)
            .with_effect(move |world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();

                match spawn_parent_variant {
                    SpawnParentVariant::HitterEntityIsSpawnParent => {
                        let mut spawn_parents = world.write_storage::<SpawnParent>();
                        spawn_parents
                            .insert(entity_from, SpawnParent::new(entity_to))
                            .expect("Failed to insert `SpawnParent` component.");
                    }
                    SpawnParentVariant::HitEntityIsSpawnParent => {
                        let mut spawn_parents = world.write_storage::<SpawnParent>();
                        spawn_parents
                            .insert(entity_to, SpawnParent::new(entity_from))
                            .expect("Failed to insert `SpawnParent` component.");
                    }
                    SpawnParentVariant::NoSpawnParent => {}
                }

                match teams_variant {
                    TeamsVariant::SameTeam => {
                        let mut teams = world.write_storage::<Team>();
                        teams
                            .insert(entity_from, Team::Independent(IndependentCounter::new(0)))
                            .expect("Failed to insert `Team` component.");
                        teams
                            .insert(entity_to, Team::Independent(IndependentCounter::new(0)))
                            .expect("Failed to insert `Team` component.");
                    }
                    TeamsVariant::DifferentTeam => {
                        let mut teams = world.write_storage::<Team>();
                        teams
                            .insert(entity_from, Team::Independent(IndependentCounter::new(0)))
                            .expect("Failed to insert `Team` component.");
                        teams
                            .insert(entity_to, Team::Independent(IndependentCounter::new(1)))
                            .expect("Failed to insert `Team` component.");
                    }
                    TeamsVariant::NoTeam => {}
                }

                send_event(world, collision_event(entity_from, entity_to));

                world.insert((entity_from, entity_to));
            })
            .with_assertion(move |world| {
                let (entity_from, entity_to) = *world.read_resource::<(Entity, Entity)>();
                let events_expected = events_expected_fn(entity_from, entity_to);
                assert_events(world, events_expected);
            })
            .run()
    }

    fn setup_event_reader(world: &mut World) {
        let contact_event_rid = world
            .write_resource::<EventChannel<ContactEvent>>()
            .register_reader(); // kcov-ignore

        world.insert(contact_event_rid);
    }

    fn send_event(world: &mut World, event: CollisionEvent) {
        let mut ec = world.write_resource::<EventChannel<CollisionEvent>>();
        ec.single_write(event)
    } // kcov-ignore

    fn collision_event(entity_from: Entity, entity_to: Entity) -> CollisionEvent {
        CollisionEvent::new(
            entity_from,
            entity_to,
            interaction(HitLimit::Limit(HIT_LIMIT)),
            body(),
        )
    }

    fn contact_event(entity_from: Entity, entity_to: Entity) -> ContactEvent {
        ContactEvent::new(
            entity_from,
            entity_to,
            interaction(HitLimit::Limit(HIT_LIMIT)),
            body(),
        )
    }

    fn interaction(hit_limit: HitLimit) -> Interaction {
        Interaction::new(
            InteractionKind::Hit(Hit {
                repeat_delay: HitRepeatDelay::new(4),
                hit_limit,
                ..Default::default()
            }),
            vec![],
            true,
        )
    }

    fn body() -> Volume {
        Volume::Box {
            x: 0,
            y: 0,
            z: 0,
            w: 1,
            h: 1,
            d: 1,
        }
    }

    fn assert_events(world: &mut World, contact_events_expected: Vec<ContactEvent>) {
        let contact_ec = world.read_resource::<EventChannel<ContactEvent>>();
        let mut contact_event_rid = world.write_resource::<ReaderId<ContactEvent>>();
        let contact_events = contact_ec
            .read(&mut contact_event_rid)
            .collect::<Vec<&ContactEvent>>();

        let contact_events_expected = contact_events_expected
            .iter()
            .collect::<Vec<&ContactEvent>>();
        assert_eq!(contact_events_expected, contact_events);
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum SpawnParentVariant {
        HitterEntityIsSpawnParent,
        HitEntityIsSpawnParent,
        NoSpawnParent,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum TeamsVariant {
        SameTeam,
        DifferentTeam,
        NoTeam,
    }
}
