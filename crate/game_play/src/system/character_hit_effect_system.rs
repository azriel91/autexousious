use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use character_model::loaded::CharacterHitTransitions;
use collision_model::{
    config::{Hit, Interaction, InteractionKind},
    play::HitEvent,
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Velocity;
use mirrored_model::play::Mirrored;
use object_model::play::HealthPoints;
use object_status_model::config::StunPoints;
use sequence_model::loaded::SequenceId;

const STUN_THRESHOLD_LOW: StunPoints = StunPoints(40);
const STUN_THRESHOLD_MID: StunPoints = StunPoints(80);
const STUN_THRESHOLD_HIGH: StunPoints = StunPoints(120);

/// Determines collision effects for characters.
#[derive(Debug, Default, new)]
pub struct CharacterHitEffectSystem {
    /// Reader ID for the `HitEvent` event channel.
    #[new(default)]
    hit_event_rid: Option<ReaderId<HitEvent>>,
}

/// `CharacterHitEffectSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterHitEffectSystemData<'s> {
    /// `HitEvent` channel.
    #[derivative(Debug = "ignore")]
    pub hit_ec: Read<'s, EventChannel<HitEvent>>,
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: ReadStorage<'s, Mirrored>,
    /// `CharacterHitTransitions` components.
    #[derivative(Debug = "ignore")]
    pub character_hit_transitionses: ReadStorage<'s, CharacterHitTransitions>,
    /// `HealthPoints` components.
    #[derivative(Debug = "ignore")]
    pub health_pointses: WriteStorage<'s, HealthPoints>,
    /// `StunPoints` components.
    #[derivative(Debug = "ignore")]
    pub stun_pointses: WriteStorage<'s, StunPoints>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl<'s> System<'s> for CharacterHitEffectSystem {
    type SystemData = CharacterHitEffectSystemData<'s>;

    fn run(
        &mut self,
        CharacterHitEffectSystemData {
            hit_ec,
            mirroreds,
            character_hit_transitionses,
            mut health_pointses,
            mut stun_pointses,
            mut velocities,
            mut sequence_ids,
        }: Self::SystemData,
    ) {
        // Read from channel
        hit_ec
            .read(
                self.hit_event_rid
                    .as_mut()
                    .expect("Expected reader ID to exist for CharacterHitEffectSystem."),
            )
            .for_each(|ev| {
                let mirrored = mirroreds
                    .get(ev.from)
                    .map(|mirrored| **mirrored)
                    .unwrap_or(false);

                let character_hit_transitions = character_hit_transitionses.get(ev.to);
                let health_points = health_pointses.get_mut(ev.to);
                let stun_points = stun_pointses.get_mut(ev.to);
                let velocity = velocities.get_mut(ev.to);
                let sequence_id = sequence_ids.get_mut(ev.to);

                if let (
                    Some(character_hit_transitions),
                    Some(health_points),
                    Some(stun_points),
                    Some(velocity),
                    Some(sequence_id),
                ) = (
                    character_hit_transitions,
                    health_points,
                    stun_points,
                    velocity,
                    sequence_id,
                ) {
                    // TODO: Split this system with health check system.
                    let Interaction {
                        kind:
                            InteractionKind::Hit(Hit {
                                hp_damage,
                                stun,
                                acceleration,
                                ..
                            }),
                        ..
                    } = ev.interaction;
                    if health_points.0 < hp_damage {
                        *health_points = HealthPoints(0);
                    } else {
                        (*health_points) -= hp_damage;
                    }

                    *stun_points += stun;

                    if mirrored {
                        velocity.x -= (*acceleration).x as f32;
                    } else {
                        velocity.x += (*acceleration).x as f32;
                    }
                    velocity.y += (*acceleration).y as f32;
                    velocity.z += (*acceleration).z as f32;

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
                    *sequence_id = next_sequence_id;
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
