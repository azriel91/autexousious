use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_play_model::{play::GamePlayEndTransitionDelayClock, GamePlayEvent};

/// Number of ticks to wait before accepting input to transition past game play
/// end.
pub const GAME_PLAY_END_TRANSITION_DELAY_DEFAULT: usize = 60;

/// Delays game play end transition from taking place when game play first ends.
///
/// This is an ergonomics improvement to prevent accidental transition, e.g.
/// when a game is won via a rapidfire attack.
#[derive(Debug, Default, new)]
pub struct GamePlayEndTransitionDelaySystem {
    /// Reader ID for the `GamePlayEvent` event channel.
    #[new(default)]
    game_play_event_rid: Option<ReaderId<GamePlayEvent>>,
}

/// `GamePlayEndTransitionDelaySystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GamePlayEndTransitionDelaySystemData<'s> {
    /// `GamePlayEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_play_ec: Read<'s, EventChannel<GamePlayEvent>>,
    /// `GamePlayEndTransitionDelayClock` resource.
    #[derivative(Debug = "ignore")]
    pub game_play_end_transition_delay_clock: Write<'s, GamePlayEndTransitionDelayClock>,
}

impl<'s> System<'s> for GamePlayEndTransitionDelaySystem {
    type SystemData = GamePlayEndTransitionDelaySystemData<'s>;

    fn run(
        &mut self,
        GamePlayEndTransitionDelaySystemData {
            game_play_ec,
            mut game_play_end_transition_delay_clock,
        }: Self::SystemData,
    ) {
        game_play_end_transition_delay_clock.tick();

        let game_play_event_rid = self
            .game_play_event_rid
            .as_mut()
            .expect("Expected `game_play_event_rid` field to be set.");

        let game_play_ended = game_play_ec
            .read(game_play_event_rid)
            .any(|ev| *ev == GamePlayEvent::End);

        if game_play_ended {
            *game_play_end_transition_delay_clock =
                GamePlayEndTransitionDelayClock::new(GAME_PLAY_END_TRANSITION_DELAY_DEFAULT);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.game_play_event_rid = Some(
            world
                .fetch_mut::<EventChannel<GamePlayEvent>>()
                .register_reader(),
        );
    }
}
