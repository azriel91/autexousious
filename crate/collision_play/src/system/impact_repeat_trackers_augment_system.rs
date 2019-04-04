use amethyst::{
    ecs::{Entity, Read, Resources, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{
    config::{Impact, ImpactRepeatDelay, Interaction, InteractionKind},
    play::{CollisionEvent, ImpactRepeatClock, ImpactRepeatTracker, ImpactRepeatTrackers},
};
use derive_new::new;
use logic_clock::LogicClock;
use typename_derive::TypeName;

/// Creates `ImpactRepeatTrackers`s for new `Impact` collisions.
///
/// This attaches ?? to the entity with the `Interaction`.
#[derive(Debug, Default, TypeName, new)]
pub struct ImpactRepeatTrackersAugmentSystem {
    /// Reader ID for the `CollisionEvent` event channel.
    #[new(default)]
    collision_event_rid: Option<ReaderId<CollisionEvent>>,
}

type ImpactRepeatTrackersAugmentSystemData<'s> = (
    Read<'s, EventChannel<CollisionEvent>>,
    WriteStorage<'s, ImpactRepeatTrackers>,
);

impl ImpactRepeatTrackersAugmentSystem {
    fn impact_repeat_tracker(
        entity_to: Entity,
        repeat_delay: ImpactRepeatDelay,
    ) -> ImpactRepeatTracker {
        let impact_repeat_clock = ImpactRepeatClock::new(LogicClock::new(*repeat_delay as usize));
        ImpactRepeatTracker::new(entity_to, impact_repeat_clock)
    }
}

impl<'s> System<'s> for ImpactRepeatTrackersAugmentSystem {
    type SystemData = ImpactRepeatTrackersAugmentSystemData<'s>;

    fn run(&mut self, (collision_ec, mut impact_repeat_trackerses): Self::SystemData) {
        // Read from channel
        collision_ec
            .read(
                self.collision_event_rid
                    .as_mut()
                    .expect("Expected reader ID to exist for ImpactRepeatTrackersAugmentSystem."),
            )
            .for_each(|ev| {
                // Only add trackers for `Impact` interactions.
                let Interaction {
                    kind: InteractionKind::Impact(Impact { repeat_delay, .. }),
                    ..
                } = ev.interaction;

                // This assumes `ev.to` is the hit object entity. If we have a separate
                // entity for each `Body`, then this assumption breaks, and we need to
                // traverse the entity hierarchy to find the object entity.
                let hit_object = ev.to;

                match impact_repeat_trackerses.get_mut(ev.from) {
                    Some(impact_repeat_trackers) => {
                        if impact_repeat_trackers
                            .iter()
                            .all(|impact_repeat_tracker| impact_repeat_tracker.entity != hit_object)
                        {
                            let impact_repeat_tracker =
                                Self::impact_repeat_tracker(hit_object, repeat_delay);
                            impact_repeat_trackers.push(impact_repeat_tracker);
                        }
                    }
                    None => {
                        let impact_repeat_tracker =
                            Self::impact_repeat_tracker(hit_object, repeat_delay);
                        let impact_repeat_trackers =
                            ImpactRepeatTrackers::new(vec![impact_repeat_tracker]);
                        impact_repeat_trackerses
                            .insert(ev.from, impact_repeat_trackers)
                            .expect("Failed to insert `ImpactRepeatTrackers`.");
                    }
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.collision_event_rid = Some(
            res.fetch_mut::<EventChannel<CollisionEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::{
        config::{Impact, ImpactRepeatDelay, Interaction, InteractionKind},
        play::{CollisionEvent, ImpactRepeatClock, ImpactRepeatTracker, ImpactRepeatTrackers},
    };
    use logic_clock::LogicClock;
    use shape_model::Volume;

    use super::ImpactRepeatTrackersAugmentSystem;

    #[test]
    fn inserts_impact_repeat_trackers_for_attacker() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ImpactRepeatTrackersAugmentSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();

                let event = CollisionEvent::new(entity_from, entity_to, interaction(), body());
                send_event(world, event);

                world.add_resource((entity_from, entity_to));
            })
            .with_assertion(|world| {
                let (entity_from, entity_to) = *world.read_resource::<(Entity, Entity)>();
                let impact_repeat_trackerses = world.read_storage::<ImpactRepeatTrackers>();
                let impact_repeat_trackers = impact_repeat_trackerses.get(entity_from);

                assert_eq!(
                    Some(&ImpactRepeatTrackers::new(vec![ImpactRepeatTracker::new(
                        entity_to,
                        ImpactRepeatClock::new(LogicClock::new(4))
                    )])),
                    impact_repeat_trackers
                );
            })
            .run()
    }

    #[test]
    fn inserts_impact_repeat_tracker_for_different_target() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ImpactRepeatTrackersAugmentSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to_0 = world.create_entity().build();
                let entity_to_1 = world.create_entity().build();

                let event = CollisionEvent::new(entity_from, entity_to_0, interaction(), body());
                send_event(world, event);

                world.add_resource((entity_from, entity_to_0, entity_to_1));
            })
            .with_assertion(|world| {
                let (entity_from, entity_to_0, _entity_to_1) =
                    *world.read_resource::<(Entity, Entity, Entity)>();
                let impact_repeat_trackerses = world.read_storage::<ImpactRepeatTrackers>();
                let impact_repeat_trackers = impact_repeat_trackerses.get(entity_from);

                assert_eq!(
                    Some(&ImpactRepeatTrackers::new(vec![ImpactRepeatTracker::new(
                        entity_to_0,
                        ImpactRepeatClock::new(LogicClock::new(4))
                    )])),
                    impact_repeat_trackers
                );
            })
            .with_effect(|world| {
                let (entity_from, _entity_to_0, entity_to_1) =
                    *world.read_resource::<(Entity, Entity, Entity)>();

                let event = CollisionEvent::new(entity_from, entity_to_1, interaction(), body());
                send_event(world, event);
            })
            .with_assertion(|world| {
                let (entity_from, entity_to_0, entity_to_1) =
                    *world.read_resource::<(Entity, Entity, Entity)>();
                let impact_repeat_trackerses = world.read_storage::<ImpactRepeatTrackers>();
                let impact_repeat_trackers = impact_repeat_trackerses.get(entity_from);

                assert_eq!(
                    Some(&ImpactRepeatTrackers::new(vec![
                        ImpactRepeatTracker::new(
                            entity_to_0,
                            ImpactRepeatClock::new(LogicClock::new(4))
                        ),
                        ImpactRepeatTracker::new(
                            entity_to_1,
                            ImpactRepeatClock::new(LogicClock::new(4))
                        )
                    ])),
                    impact_repeat_trackers
                );
            })
            .run()
    }

    fn send_event(world: &mut World, event: CollisionEvent) {
        let mut ec = world.write_resource::<EventChannel<CollisionEvent>>();
        ec.single_write(event)
    }

    fn interaction() -> Interaction {
        Interaction::new(
            InteractionKind::Impact(Impact::new(ImpactRepeatDelay::new(4), 0, 0)),
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
}
