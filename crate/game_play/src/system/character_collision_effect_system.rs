use amethyst::{
    ecs::{Read, Resources, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{config::Interaction, play::CollisionEvent};
use derive_new::new;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{HealthPoints, SequenceStatus},
};
use typename_derive::TypeName;

/// Determines collision effects for characters.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterCollisionEffectSystem {
    /// Reader ID for the `CollisionEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<CollisionEvent>>,
}

type CharacterCollisionEffectSystemData<'s> = (
    Read<'s, EventChannel<CollisionEvent>>,
    WriteStorage<'s, HealthPoints>,
    WriteStorage<'s, CharacterSequenceId>,
    WriteStorage<'s, SequenceStatus>,
);

impl<'s> System<'s> for CharacterCollisionEffectSystem {
    type SystemData = CharacterCollisionEffectSystemData<'s>;

    fn run(
        &mut self,
        (collision_ec, mut health_pointses, mut character_sequence_ids, mut sequence_statuses): Self::SystemData,
    ) {
        // Read from channel
        collision_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for CharacterCollisionEffectSystem."),
            )
            .for_each(|ev| {
                // Fetch health points of the object that is hit.
                let health_points = health_pointses.get_mut(ev.to);
                let character_sequence_id = character_sequence_ids.get_mut(ev.to);
                let sequence_status = sequence_statuses.get_mut(ev.to);

                if let (Some(health_points), Some(character_sequence_id), Some(sequence_status)) =
                    (health_points, character_sequence_id, sequence_status)
                {
                    // TODO: Select damage sequence based on status.
                    // TODO: Split this system with health check system.
                    let Interaction::Physical { hp_damage, .. } = ev.interaction;
                    if health_points.0 < hp_damage {
                        *health_points = HealthPoints(0);
                    } else {
                        (*health_points) -= hp_damage;
                    }

                    let next_sequence_id = if *health_points == 0 {
                        CharacterSequenceId::FallForwardAscend
                    } else {
                        if *character_sequence_id == CharacterSequenceId::Flinch0 {
                            CharacterSequenceId::Flinch1
                        } else {
                            CharacterSequenceId::Flinch0
                        }
                    };

                    // Set sequence id
                    *character_sequence_id = next_sequence_id;
                    *sequence_status = SequenceStatus::Begin;
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(
            res.fetch_mut::<EventChannel<CollisionEvent>>()
                .register_reader(),
        );
    }
}
