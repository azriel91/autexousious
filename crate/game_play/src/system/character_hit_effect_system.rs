use amethyst::{
    ecs::{Read, ReadStorage, System, SystemData, World, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use character_model::loaded::CharacterHitTransitions;
use collision_model::{
    config::{Hit, Interaction, InteractionKind},
    play::HitEvent,
};
use derive_new::new;
use object_model::play::HealthPoints;
use object_status_model::config::StunPoints;
use sequence_model::loaded::SequenceId;
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
    ReadStorage<'s, CharacterHitTransitions>,
    WriteStorage<'s, HealthPoints>,
    WriteStorage<'s, StunPoints>,
    WriteStorage<'s, SequenceId>,
);

impl<'s> System<'s> for CharacterHitEffectSystem {
    type SystemData = CharacterHitEffectSystemData<'s>;

    fn run(
        &mut self,
        (
            hit_ec,
            character_hit_transitionses,
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
                let character_hit_transitions = character_hit_transitionses.get(ev.to);
                let health_points = health_pointses.get_mut(ev.to);
                let stun_points = stun_pointses.get_mut(ev.to);
                let character_sequence_id = character_sequence_ids.get_mut(ev.to);

                if let (
                    Some(character_hit_transitions),
                    Some(health_points),
                    Some(stun_points),
                    Some(character_sequence_id),
                ) = (
                    character_hit_transitions,
                    health_points,
                    stun_points,
                    character_sequence_id,
                ) {
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
                        character_hit_transitions.falling
                    } else if *stun_points < STUN_THRESHOLD_LOW {
                        character_hit_transitions.low_stun
                    } else if *stun_points < STUN_THRESHOLD_MID {
                        character_hit_transitions.mid_stun
                    } else if *stun_points < STUN_THRESHOLD_HIGH {
                        character_hit_transitions.high_stun
                    } else {
                        character_hit_transitions.falling
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
