#![windows_subsystem = "windows"]

//! Opens an empty window.

extern crate amethyst;
#[macro_use]
extern crate application;
extern crate application_input;
extern crate application_robot;
extern crate application_ui;
extern crate character_selection;
extern crate game_input;
extern crate game_mode_menu;
extern crate game_play;
extern crate loading;
#[macro_use]
extern crate log;
extern crate map_loading;
extern crate object_loading;
extern crate object_model;
extern crate stdio_view;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::process;
use std::time::Duration;

use amethyst::{
    animation::AnimationBundle,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{Bindings, InputBundle},
    prelude::*,
    renderer::{
        ColorMask, DisplayConfig, DrawSprite, Pipeline, RenderBundle, SpriteRender, Stage, ALPHA,
    },
    ui::{DrawUi, UiBundle},
    LogLevelFilter, LoggerConfig,
};
use application::resource::{
    dir::{self, assets_dir},
    {self, load_in},
};
use application_robot::RobotState;
use character_selection::CharacterSelectionBundle;
use game_input::{PlayerActionControl, PlayerAxisControl};
use game_mode_menu::GameModeMenuState;
use game_play::GamePlayBundle;
use map_loading::MapLoadingBundle;
use object_loading::ObjectLoadingBundle;
use object_model::config::object::CharacterSequenceId;
use stdio_view::StdioViewBundle;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Will")]
struct Opt {
    #[structopt(long = "headless", help = "Run headlessly (no GUI)")]
    headless: bool,
}

fn run(opt: &Opt) -> Result<(), amethyst::Error> {
    amethyst::start_logger(LoggerConfig {
        level_filter: if cfg!(debug_assertions) {
            LogLevelFilter::Debug
        } else {
            LogLevelFilter::Info
        },
        ..Default::default()
    });

    let assets_dir = assets_dir(Some(development_base_dirs!()))?;

    let game_mode_menu_state = GameModeMenuState::new();
    let loading_state = loading::State::new(assets_dir.clone(), Box::new(game_mode_menu_state));
    let state = RobotState::new(Box::new(loading_state));

    let mut game_data = GameDataBuilder::default();
    if !opt.headless {
        let display_config = load_in::<DisplayConfig, _>(
            dir::RESOURCES,
            "display_config.ron",
            resource::Format::Ron,
            Some(development_base_dirs!()),
        )?;

        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0., 0., 0., 0.], 1.)
                .with_pass(DrawSprite::new().with_transparency(ColorMask::all(), ALPHA, None))
                .with_pass(DrawUi::new()),
        );

        // We parse it ourselves, because Amethyst silently fails if it fails to parse.
        let input_bindings = load_in::<Bindings<PlayerAxisControl, PlayerActionControl>, _>(
            dir::RESOURCES,
            "input.ron",
            resource::Format::Ron,
            Some(development_base_dirs!()),
        )?;
        debug!("Axes bindings: {:?}", &input_bindings.axes());
        debug!("Action bindings: {:?}", &input_bindings.actions());

        // `InputBundle` provides `InputHandler<A, B>`, needed by the `UiBundle` for mouse events.
        // `UiBundle` registers `Loader<FontAsset>`, needed by `ApplicationUiBundle`.
        game_data = game_data
            // Provides sprite animation
            .with_bundle(AnimationBundle::<CharacterSequenceId, SpriteRender>::new(
                "character_animation_control_system",
                "character_sampler_interpolation_system",
            ))?
            .with_bundle(AnimationBundle::<u32, SpriteRender>::new(
                "animation_control_system",
                "sampler_interpolation_system",
            ))?
            // Handles transformations of textures
            .with_bundle(
                TransformBundle::new()
                    .with_dep(&[
                        "character_animation_control_system",
                        "character_sampler_interpolation_system",
                        "animation_control_system",
                        "sampler_interpolation_system",
                    ]),
            )?
            .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
            .with_bundle(InputBundle::<PlayerAxisControl, PlayerActionControl>::new()
                .with_bindings(input_bindings))?
            .with_bundle(UiBundle::<PlayerAxisControl, PlayerActionControl>::new())?
            .with_bundle(StdioViewBundle::new())?
            .with_bundle(MapLoadingBundle::new())?
            .with_bundle(ObjectLoadingBundle::new())?
            .with_bundle(CharacterSelectionBundle::new())?
            .with_bundle(GamePlayBundle::new())?;
    }

    let mut app = Application::build(assets_dir, state)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_micros(1000)),
            60,
        )
        .build(game_data)?;

    app.run();

    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = run(&opt) {
        println!("Failed to execute example: {}", e);
        process::exit(1);
    }
}
