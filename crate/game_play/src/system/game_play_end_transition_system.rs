use amethyst::{
    ecs::{Join, Read, ReadStorage, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use game_input::ControllerInput;
use game_play_model::{play::GamePlayEndTransitionDelayClock, GamePlayEvent, GamePlayStatus};
use tracker::Last;
use typename_derive::TypeName;

/// Detects the end of a game play round, and fires a `GamePlayEvent::End`.
///
/// In the future this will be type parameterized to specify the detection function.
#[derive(Debug, Default, TypeName, new)]
pub struct GamePlayEndTransitionSystem;

/// `GamePlayEndTransitionSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GamePlayEndTransitionSystemData<'s> {
    /// `GamePlayEndTransitionDelayClock` resource.
    #[derivative(Debug = "ignore")]
    pub game_play_end_transition_delay_clock: Read<'s, GamePlayEndTransitionDelayClock>,
    /// `GamePlayStatus` resource.
    #[derivative(Debug = "ignore")]
    pub game_play_status: Write<'s, GamePlayStatus>,
    /// `Last<ControllerInput>` components.
    #[derivative(Debug = "ignore")]
    pub last_controller_inputs: ReadStorage<'s, Last<ControllerInput>>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `GamePlayEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_play_ec: Write<'s, EventChannel<GamePlayEvent>>,
}

impl<'s> System<'s> for GamePlayEndTransitionSystem {
    type SystemData = GamePlayEndTransitionSystemData<'s>;

    fn run(
        &mut self,
        GamePlayEndTransitionSystemData {
            game_play_end_transition_delay_clock,
            mut game_play_status,
            last_controller_inputs,
            controller_inputs,
            mut game_play_ec,
        }: Self::SystemData,
    ) {
        if game_play_end_transition_delay_clock.is_complete()
            && *game_play_status == GamePlayStatus::Ended
        {
            // Transition when someone presses attack
            let should_transition = (&last_controller_inputs, &controller_inputs).join().fold(
                false,
                |should_transition, (last_controller_input, controller_input)| {
                    should_transition || (!last_controller_input.attack && controller_input.attack)
                },
            );

            if should_transition {
                *game_play_status = GamePlayStatus::None;

                game_play_ec.single_write(GamePlayEvent::EndStats);
            }
        }
    }
}
