use amethyst::{
    ecs::{Read, System, SystemData, World, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use character_model::config::CharacterSequenceId;
use collision_model::{
    config::{Hit, Interaction, InteractionKind},
    play::HitEvent,
};
use derive_new::new;
use object_model::play::HealthPoints;
use object_status_model::config::StunPoints;
use typename_derive::TypeName;

const STUN_THRESHOLD_LOW: StunPoints = StunPoints(40);
const STUN_THRESHOLD_MID: StunPoints = StunPoints(80);
const STUN_THRESHOLD_HIGH: StunPoints = StunPoints(120);

/// Determines collision effects for characters.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterHitEffectSystem {
    /// Reader ID for the `HitEvent` event channel.
    #[new(default)]
    hit_event_rid: Option<ReaderId<HitEvent>>,
}

type CharacterHitEffectSystemData<'s> = (
    Read<'s, EventChannel<HitEvent>>,
    WriteStorage<'s, HealthPoints>,
    WriteStorage<'s, StunPoints>,
    WriteStorage<'s, CharacterSequenceId>,
);

impl<'s> System<'s> for CharacterHitEffectSystem {
    type SystemData = CharacterHitEffectSystemData<'s>;

    fn run(
        &mut self,
        (
            hit_ec,
            mut health_pointses,
            mut stun_pointses,
            mut character_sequence_ids,
        ): Self::SystemData,
    ) {
        // Read from channel
        hit_ec
            .read(
                self.hit_event_rid
                    .as_mut()
                    .expect("Expected reader ID to exist for CharacterHitEffectSystem."),
            )
            .for_each(|ev| {
                // Fetch health points of the object that is hit.
                let health_points = health_pointses.get_mut(ev.to);
                let stun_points = stun_pointses.get_mut(ev.to);
                let character_sequence_id = character_sequence_ids.get_mut(ev.to);

                if let (Some(health_points), Some(stun_points), Some(character_sequence_id)) =
                    (health_points, stun_points, character_sequence_id)
                {
                    // TODO: Split this system with health check system.
                    let Interaction {
                        kind:
                            InteractionKind::Hit(Hit {
                                hp_damage, stun, ..
                            }),
                        ..
                    } = ev.interaction;
                    if health_points.0 < hp_damage {
                        *health_points = HealthPoints(0);
                    } else {
                        (*health_points) -= hp_damage;
                    }

                    *stun_points += stun;

                    let next_sequence_id = if *health_points == 0 {
                        CharacterSequenceId::FallForwardAscend
                    } else if *stun_points < STUN_THRESHOLD_LOW {
                        CharacterSequenceId::Flinch0
                    } else if *stun_points < STUN_THRESHOLD_MID {
                        CharacterSequenceId::Flinch1
                    } else if *stun_points < STUN_THRESHOLD_HIGH {
                        CharacterSequenceId::Dazed
                    } else {
                        CharacterSequenceId::FallForwardAscend
                    };

                    // Set sequence id
                    *character_sequence_id = next_sequence_id;
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.hit_event_rid = Some(
            world
                .fetch_mut::<EventChannel<HitEvent>>()
                .register_reader(),
        );
    }
}
