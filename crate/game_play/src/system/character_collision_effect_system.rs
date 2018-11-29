use amethyst::{
    ecs::{Read, Resources, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{config::Interaction, play::CollisionEvent};
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, HealthPoints, ObjectStatus, SequenceStatus},
};
use typename::TypeName;

/// Determines collision effects for characters.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterCollisionEffectSystem {
    /// Reader ID for the `CollisionEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<CollisionEvent>>,
}

type CharacterCollisionEffectSystemData<'s> = (
    Read<'s, EventChannel<CollisionEvent>>,
    WriteStorage<'s, CharacterStatus>,
    WriteStorage<'s, ObjectStatus<CharacterSequenceId>>,
    WriteStorage<'s, SequenceStatus>,
);

impl<'s> System<'s> for CharacterCollisionEffectSystem {
    type SystemData = CharacterCollisionEffectSystemData<'s>;

    fn run(
        &mut self,
        (collision_ec, mut character_statuses, mut object_statuses, mut sequence_statuses): Self::SystemData,
    ) {
        // Read from channel
        collision_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for CharacterCollisionEffectSystem."),
            )
            .for_each(|ev| {
                // Fetch CharacterStatus for entity
                let character_status = character_statuses.get_mut(ev.to);
                let object_status = object_statuses.get_mut(ev.to);
                let sequence_status = sequence_statuses.get_mut(ev.to);

                if let (Some(character_status), Some(object_status), Some(sequence_status)) =
                    (character_status, object_status, sequence_status)
                {
                    // TODO: Select damage sequence based on status.
                    // TODO: Split this system with health check system.
                    let Interaction::Physical { hp_damage, .. } = ev.interaction;
                    if character_status.hp.0 < hp_damage {
                        character_status.hp = HealthPoints(0);
                    } else {
                        character_status.hp -= hp_damage;
                    }

                    let next_sequence_id = if character_status.hp == 0 {
                        CharacterSequenceId::FallForwardAscend
                    } else {
                        if object_status.sequence_id == CharacterSequenceId::Flinch0 {
                            CharacterSequenceId::Flinch1
                        } else {
                            CharacterSequenceId::Flinch0
                        }
                    };

                    // Set sequence id
                    object_status.sequence_id = next_sequence_id;
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
