use std::collections::HashMap;

use amethyst::{
    ecs::{Join, ReadStorage, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use game_play_model::{GamePlayEvent, GamePlayStatus};
use game_stats_model::play::{WinOutcome, WinStatus};
use object_model::play::HealthPoints;
use team_model::play::Team;

/// Detects the end of a game play round, and fires a `GamePlayEvent::End`.
///
/// In the future this will be type parameterized to specify the detection
/// function.
#[derive(Debug, Default, new)]
pub struct GamePlayEndDetectionSystem {
    /// Pre-allocated `HashMap` to track number of alive players.
    #[new(default)]
    team_alive_counter: HashMap<Team, u32>,
}

/// `GamePlayEndDetectionSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GamePlayEndDetectionSystemData<'s> {
    /// `GamePlayStatus` resource.
    #[derivative(Debug = "ignore")]
    pub game_play_status: Write<'s, GamePlayStatus>,
    /// `GamePlayEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_play_ec: Write<'s, EventChannel<GamePlayEvent>>,
    /// `Team` components.
    #[derivative(Debug = "ignore")]
    pub teams: ReadStorage<'s, Team>,
    /// `HealthPoints` components.
    #[derivative(Debug = "ignore")]
    pub health_pointses: ReadStorage<'s, HealthPoints>,
    /// `WinStatus` resource.
    #[derivative(Debug = "ignore")]
    pub win_status: Write<'s, WinStatus>,
}

impl GamePlayEndDetectionSystem {
    /// Computes and returns the `WinStatus` if the game has ended.
    fn win_status(
        &mut self,
        GamePlayEndDetectionSystemData {
            teams,
            health_pointses,
            ..
        }: &mut GamePlayEndDetectionSystemData,
    ) -> Option<WinStatus> {
        let team_alive_count = self.team_alive_count(&teams, &health_pointses);
        if team_alive_count == 0 {
            let win_outcome = WinOutcome::Draw;
            let win_status = WinStatus::new(win_outcome);
            Some(win_status)
        } else if team_alive_count == 1 {
            let winning_team = self
                .team_alive_counter
                .keys()
                .next()
                .copied()
                .expect("Expected `Team` entry to exist.");
            let win_outcome = WinOutcome::WinLoss { winning_team };
            let win_status = WinStatus::new(win_outcome);
            Some(win_status)
        } else {
            None
        }
    }

    /// Returns the number of teams remaining in play.
    fn team_alive_count(
        &mut self,
        teams: &ReadStorage<'_, Team>,
        health_pointses: &ReadStorage<'_, HealthPoints>,
    ) -> usize {
        self.team_alive_counter.clear();

        // Game ends when there is one or less teams remaining
        (teams, health_pointses)
            .join()
            .for_each(|(team, health_points)| {
                if *health_points > 0 {
                    let alive_count = self.team_alive_counter.entry(*team).or_insert(0);
                    *alive_count += 1;
                };
            });

        self.team_alive_counter.len()
    }
}

impl<'s> System<'s> for GamePlayEndDetectionSystem {
    type SystemData = GamePlayEndDetectionSystemData<'s>;

    fn run(&mut self, mut system_data: Self::SystemData) {
        match *system_data.game_play_status {
            GamePlayStatus::Playing => {
                if let Some(win_status) = self.win_status(&mut system_data) {
                    *system_data.win_status = win_status;
                    *system_data.game_play_status = GamePlayStatus::Ended;
                    system_data.game_play_ec.single_write(GamePlayEvent::End);
                }
            }
            GamePlayStatus::Ended => {
                if self.win_status(&mut system_data).is_none() {
                    *system_data.win_status = WinStatus::default();
                    *system_data.game_play_status = GamePlayStatus::Playing;
                }
            }
            GamePlayStatus::None | GamePlayStatus::Paused => {}
        }
    }
}
