use std::convert::TryInto;

use amethyst::{
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
    ui::{Anchor, UiText, UiTransform},
};
use application_ui::{FontVariant, Theme};
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_input_model::InputConfig;
use game_play_model::{
    play::GamePlayStatusEntity, GamePlayEntity, GamePlayEntityId, GamePlayEvent,
};
use game_stats_model::play::{WinOutcome, WinStatus};
use team_model::play::Team;
use typename_derive::TypeName;

const FONT_COLOUR_NEUTRAL: [f32; 4] = [0.8, 0.9, 1., 1.];
const FONT_SIZE_WIDGET: f32 = 50.;
const LABEL_WIDTH: f32 = 600.;
const LABEL_HEIGHT: f32 = 100.;

/// Displays the status of game play.
///
/// Intended for displaying when the game is paused, or when the game ends (winning team).
#[derive(Debug, Default, TypeName, new)]
pub struct GamePlayStatusDisplaySystem {
    /// Reader ID for the `GamePlayEvent` event channel.
    #[new(default)]
    game_play_event_rid: Option<ReaderId<GamePlayEvent>>,
}

/// `GamePlayStatusDisplaySystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GamePlayStatusDisplaySystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `GamePlayEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_play_ec: Read<'s, EventChannel<GamePlayEvent>>,
    /// `GamePlayStatusEntity` components.
    #[derivative(Debug = "ignore")]
    pub game_play_status_entities: WriteStorage<'s, GamePlayStatusEntity>,
    /// `GamePlayEntity` components.
    #[derivative(Debug = "ignore")]
    pub game_play_entities: WriteStorage<'s, GamePlayEntity>,
    /// `WinStatus` resource.
    #[derivative(Debug = "ignore")]
    pub win_status: Read<'s, WinStatus>,
    /// `Team` components.
    #[derivative(Debug = "ignore")]
    pub teams: ReadStorage<'s, Team>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `InputConfig` resource.
    #[derivative(Debug = "ignore")]
    pub input_config: Read<'s, InputConfig>,

    // Resources needed to display text.
    /// `Theme` resource.
    #[derivative(Debug = "ignore")]
    pub theme: ReadExpect<'s, Theme>,
    /// `UiTransform` components.
    #[derivative(Debug = "ignore")]
    pub ui_transforms: WriteStorage<'s, UiTransform>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
}

impl GamePlayStatusDisplaySystem {
    /// Deletes existing entities used to display game play status.
    fn delete_existing(
        entities: &Entities<'_>,
        game_play_status_entities: &mut WriteStorage<'_, GamePlayStatusEntity>,
    ) {
        (entities, game_play_status_entities)
            .join()
            .for_each(|(entity, _)| {
                entities
                    .delete(entity)
                    .expect("Failed to delete `GamePlayStatus` entity");
            });
    }

    fn win_status_text(
        win_status: WinStatus,
        teams: &ReadStorage<'_, Team>,
        input_controlleds: &ReadStorage<'_, InputControlled>,
        input_config: &InputConfig,
    ) -> String {
        match win_status.outcome {
            WinOutcome::None => String::from("Ongoing Match"),
            WinOutcome::WinLoss { winning_team } => {
                let winner = (teams, input_controlleds)
                    .join()
                    .filter(|(team, _)| **team == winning_team)
                    .next()
                    .map(|(team, input_controlled)| match team {
                        Team::Independent(..) => {
                            let controller_id = input_controlled.controller_id;
                            input_config
                                .controller_configs
                                .get_index(
                                    controller_id
                                        .try_into()
                                        .expect("Failed to convert `u32` into `usize`"),
                                )
                                .map(|(name, _)| name.clone())
                                .unwrap_or_else(|| {
                                    panic!(
                                        "Expected `InputConfig` to have at least \
                                         {} controllers.",
                                        controller_id + 1
                                    )
                                })
                        }
                        Team::Number(team_counter) => format!("Team {}", team_counter),
                    })
                    .expect("Expected entity for winning team to exist.");
                format!("Winner: {}", winner)
            }
            WinOutcome::Draw => String::from("Draw"),
        }
    }
}

impl<'s> System<'s> for GamePlayStatusDisplaySystem {
    type SystemData = GamePlayStatusDisplaySystemData<'s>;

    fn run(
        &mut self,
        GamePlayStatusDisplaySystemData {
            entities,
            game_play_ec,
            win_status,
            teams,
            input_controlleds,
            input_config,
            mut game_play_status_entities,
            mut game_play_entities,
            theme,
            mut ui_transforms,
            mut ui_texts,
        }: Self::SystemData,
    ) {
        let game_play_event_rid = self
            .game_play_event_rid
            .as_mut()
            .expect("Expected `game_play_event_rid` field to be set.");

        game_play_ec
            .read(game_play_event_rid)
            .copied()
            .for_each(|ev| {
                Self::delete_existing(&entities, &mut game_play_status_entities);

                if ev == GamePlayEvent::End {
                    let font = theme
                        .fonts
                        .get(&FontVariant::Bold)
                        .expect("Failed to get regular font handle.");

                    let x = -LABEL_WIDTH / 2.;
                    let y = 0.;
                    let z = 1.;

                    let ui_transform = UiTransform::new(
                        String::from("game_play_status_text"),
                        Anchor::Middle,
                        Anchor::MiddleLeft,
                        x,
                        y,
                        z,
                        LABEL_WIDTH,
                        LABEL_HEIGHT,
                    );

                    let win_status_text = Self::win_status_text(
                        *win_status,
                        &teams,
                        &input_controlleds,
                        &input_config,
                    );

                    let ui_text = UiText::new(
                        font.clone(),
                        win_status_text,
                        FONT_COLOUR_NEUTRAL,
                        FONT_SIZE_WIDGET,
                    );

                    entities
                        .build_entity()
                        .with(
                            GamePlayEntity::new(GamePlayEntityId),
                            &mut game_play_entities,
                        )
                        .with(GamePlayStatusEntity, &mut game_play_status_entities)
                        .with(ui_transform, &mut ui_transforms)
                        .with(ui_text, &mut ui_texts)
                        .build();
                }
            });
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
