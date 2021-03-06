use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::{Read, System, SystemData, World},
    shrev::{EventChannel, ReaderId},
};
use collision_audio_model::{config::CollisionSfxId, loaded::CollisionSfxMap};
use collision_model::{
    config::{Hit, Interaction, InteractionKind},
    play::HitEvent,
};
use derive_new::new;

/// Default volume to play sounds at.
const VOLUME: f32 = 1.0;

/// Plays a sound for `Hit` collisions.
#[derive(Debug, Default, new)]
pub struct HitSfxSystem {
    /// Reader ID for the `HitEvent` event channel.
    #[new(default)]
    hit_event_rid: Option<ReaderId<HitEvent>>,
}

type HitSfxSystemData<'s> = (
    Read<'s, EventChannel<HitEvent>>,
    Read<'s, CollisionSfxMap>,
    Read<'s, AssetStorage<Source>>,
    Option<Read<'s, Output>>,
);

impl<'s> System<'s> for HitSfxSystem {
    type SystemData = HitSfxSystemData<'s>;

    fn run(&mut self, (hit_ec, collision_sfx_map, source_assets, output): Self::SystemData) {
        // Make sure we empty the event channel, even if we don't have an output device.
        let events_iterator = hit_ec.read(
            self.hit_event_rid
                .as_mut()
                .expect("Expected reader ID to exist for HitSfxSystem."),
        );

        if let Some(output) = output {
            events_iterator.for_each(|ev| {
                // Play sound for `Hit` interactions.
                let Interaction {
                    kind: InteractionKind::Hit(Hit { .. }),
                    ..
                } = ev.interaction;

                let hit_sfx = collision_sfx_map
                    .get(&CollisionSfxId::HitNormal)
                    .and_then(|hit_sfx_handle| source_assets.get(hit_sfx_handle));

                if let Some(hit_sfx) = hit_sfx {
                    output.play_once(hit_sfx, VOLUME);
                }
            });
        }
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
