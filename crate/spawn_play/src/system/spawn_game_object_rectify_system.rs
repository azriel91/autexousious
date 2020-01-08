use amethyst::{
    ecs::{Entity, Read, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::{Position, Velocity};
use mirrored_model::play::Mirrored;
use sequence_model::loaded::SequenceId;
use spawn_model::{
    loaded::Spawn,
    play::{SpawnEvent, SpawnParent},
};
use team_model::play::Team;

/// Spawns `GameObject`s.
#[derive(Debug, Default, new)]
pub struct SpawnGameObjectRectifySystem {
    /// Reader ID for the `SpawnEvent` channel.
    #[new(default)]
    spawn_event_rid: Option<ReaderId<SpawnEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpawnGameObjectRectifySystemData<'s> {
    /// `SpawnEvent` channel.
    #[derivative(Debug = "ignore")]
    pub spawn_ec: Read<'s, EventChannel<SpawnEvent>>,
    /// `SpawnParent` components.
    #[derivative(Debug = "ignore")]
    pub spawn_parents: WriteStorage<'s, SpawnParent>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: WriteStorage<'s, Mirrored>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
    /// `Team` components.
    #[derivative(Debug = "ignore")]
    pub teams: WriteStorage<'s, Team>,
}

impl<'s> System<'s> for SpawnGameObjectRectifySystem {
    type SystemData = SpawnGameObjectRectifySystemData<'s>;

    fn run(
        &mut self,
        SpawnGameObjectRectifySystemData {
            spawn_ec,
            mut spawn_parents,
            mut positions,
            mut velocities,
            mut mirroreds,
            mut sequence_ids,
            mut teams,
        }: Self::SystemData,
    ) {
        let spawn_event_rid = self
            .spawn_event_rid
            .as_mut()
            .expect("Expected `spawn_event_rid` field to be set.");

        spawn_ec.read(spawn_event_rid).for_each(|ev| {
            let spawn = &ev.spawn;
            let entity_parent = ev.entity_parent;
            let entity_spawned = ev.entity_spawned;
            let mirrored_parent = mirroreds.get(entity_parent).copied();
            let team_parent = teams.get(entity_parent).copied();

            let position =
                Self::position_rectify(&positions, spawn, entity_parent, mirrored_parent);
            let velocity =
                Self::velocity_rectify(&velocities, spawn, entity_parent, mirrored_parent);
            let mirrored = Self::mirrored_rectify(mirrored_parent);
            let sequence_id = spawn.sequence_id;

            spawn_parents
                .insert(entity_spawned, SpawnParent::new(ev.entity_parent))
                .expect("Failed to insert `SpawnParent` component.");
            positions
                .insert(entity_spawned, position)
                .expect("Failed to insert `Position` component.");
            velocities
                .insert(entity_spawned, velocity)
                .expect("Failed to insert `Velocity` component.");
            mirroreds
                .insert(entity_spawned, mirrored)
                .expect("Failed to insert `Mirrored` component.");
            sequence_ids
                .insert(entity_spawned, sequence_id)
                .expect("Failed to insert `SequenceId` component.");
            if let Some(team) = team_parent {
                teams
                    .insert(entity_spawned, team)
                    .expect("Failed to insert `Team` component.");
            }
        });
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

impl SpawnGameObjectRectifySystem {
    /// Returns the rectified `Position<f32>` for the spawned entity.
    fn position_rectify(
        positions: &WriteStorage<'_, Position<f32>>,
        spawn: &Spawn,
        entity_parent: Entity,
        mirrored_parent: Option<Mirrored>,
    ) -> Position<f32> {
        let spawn_position = spawn.position;
        let spawn_position_x = if let Some(Mirrored(true)) = mirrored_parent {
            -spawn_position.x
        } else {
            spawn_position.x
        };
        let mut position = spawn_position;
        position.x = spawn_position_x;
        if let Some(position_parent) = positions.get(entity_parent) {
            *position += **position_parent;
        }
        position
    }

    /// Returns the rectified `Velocity<f32>` for the spawned entity.
    fn velocity_rectify(
        velocities: &WriteStorage<'_, Velocity<f32>>,
        spawn: &Spawn,
        entity_parent: Entity,
        mirrored_parent: Option<Mirrored>,
    ) -> Velocity<f32> {
        let spawn_velocity = spawn.velocity;
        let spawn_velocity_x = if let Some(Mirrored(true)) = mirrored_parent {
            -spawn_velocity.x
        } else {
            spawn_velocity.x
        };
        let mut velocity = spawn_velocity;
        velocity.x = spawn_velocity_x;
        if let Some(velocity_parent) = velocities.get(entity_parent) {
            *velocity += **velocity_parent;
        }
        velocity
    }

    /// Returns the rectified `Mirrored` for the spawned entity.
    fn mirrored_rectify(mirrored_parent: Option<Mirrored>) -> Mirrored {
        mirrored_parent.unwrap_or(Mirrored(false))
    }
}
