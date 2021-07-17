use amethyst::{
    ecs::{Read, ReadStorage, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use collision_model::play::{CollisionEvent, ContactEvent};
use derivative::Derivative;
use derive_new::new;
use spawn_model::play::SpawnParent;
use team_model::play::Team;

/// Detects whether a `ContactEvent` occurs when a `CollisionEvent` happens.
///
/// This system determines if contact happens or not -- e.g. objects on the same
/// team may or may not contact each other depending on the type of
/// `Interaction`.
#[derive(Debug, Default, new)]
pub struct ContactDetectionSystem {
    /// Reader ID for the `CollisionEvent` event channel.
    #[new(default)]
    collision_event_rid: Option<ReaderId<CollisionEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ContactDetectionSystemData<'s> {
    /// `CollisionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub collision_ec: Read<'s, EventChannel<CollisionEvent>>,
    /// `SpawnParent` components.
    #[derivative(Debug = "ignore")]
    pub spawn_parents: ReadStorage<'s, SpawnParent>,
    /// `Team` components.
    #[derivative(Debug = "ignore")]
    pub teams: ReadStorage<'s, Team>,
    /// `ContactEvent` channel.
    #[derivative(Debug = "ignore")]
    pub contact_ec: Write<'s, EventChannel<ContactEvent>>,
}

impl<'s> System<'s> for ContactDetectionSystem {
    type SystemData = ContactDetectionSystemData<'s>;

    fn run(
        &mut self,
        ContactDetectionSystemData {
            collision_ec,
            spawn_parents,
            teams,
            mut contact_ec,
        }: Self::SystemData,
    ) {
        let contact_events = collision_ec
            .read(
                self.collision_event_rid.as_mut().expect(
                    "Expected `collision_event_rid` to exist for `ContactDetectionSystem`.",
                ),
            )
            .filter(|ev| {
                // This assumes `ev.from` is the hitting object entity. If we have a separate
                // entity for each `Interaction`, then this assumption breaks, and we need to
                // traverse the entity hierarchy to find the object entity.
                let entity_hitter = ev.from;

                // This assumes `ev.to` is the hit object entity. If we have a separate
                // entity for each `Body`, then this assumption breaks, and we need to
                // traverse the entity hierarchy to find the object entity.
                let entity_hit = ev.to;

                let team_from = teams.get(entity_hitter);
                let team_to = teams.get(entity_hit);
                let dont_hit_team = if let (Some(team_from), Some(team_to)) = (team_from, team_to) {
                    team_from != team_to
                } else {
                    true
                };

                let dont_hit_spawn_parent = spawn_parents
                    .get(entity_hitter)
                    .map(|spawn_parent| spawn_parent.entity != entity_hit)
                    .unwrap_or(true);

                let dont_hit_spawned_object = spawn_parents
                    .get(entity_hit)
                    .map(|spawn_parent| spawn_parent.entity != entity_hitter)
                    .unwrap_or(true);

                dont_hit_team && dont_hit_spawn_parent && dont_hit_spawned_object
            })
            .map(|ev| ContactEvent::new(ev.from, ev.to, ev.interaction.clone(), ev.body))
            .collect::<Vec<ContactEvent>>();

        contact_ec.iter_write(contact_events);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.collision_event_rid = Some(
            world
                .fetch_mut::<EventChannel<CollisionEvent>>()
                .register_reader(),
        );
    }
}
