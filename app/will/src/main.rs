#![windows_subsystem = "windows"]

//! Opens an empty window.

extern crate amethyst;
#[macro_use]
extern crate application;
extern crate application_input;
extern crate application_robot;
extern crate application_ui;
extern crate game_input;
extern crate game_mode_menu;
extern crate loading;
extern crate stdio_view;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::process;

use amethyst::animation::AnimationBundle;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{ColorMask, DisplayConfig, DrawFlat, Material, Pipeline, PosTex,
                         RenderBundle, Stage, ALPHA};
use amethyst::ui::{DrawUi, UiBundle};
use application::resource::dir::{self, assets_dir};
use application::resource::find_in;
use application_robot::RobotState;
use game_input::{PlayerActionControl, PlayerAxisControl};
use game_mode_menu::GameModeMenuState;
use stdio_view::StdinSystem;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Free Will")]
struct Opt {
    #[structopt(long = "headless", help = "Run headlessly (no GUI)")]
    headless: bool,
}

fn run(opt: &Opt) -> Result<(), amethyst::Error> {
    let assets_dir = assets_dir(Some(development_base_dirs!()))?;

    let game_mode_menu_state = GameModeMenuState::new();
    let loading_state = loading::State::new(assets_dir.clone(), Box::new(game_mode_menu_state));
    let state = RobotState::new(Box::new(loading_state));

    let mut app_builder = Application::build(assets_dir, state)?.with::<StdinSystem>(
        StdinSystem::new(),
        "StdinSystem",
        &[],
    );

    if !opt.headless {
        let display_config = DisplayConfig::load(
            find_in(
                dir::RESOURCES,
                "display_config.ron",
                Some(development_base_dirs!()),
            ).unwrap(),
        );

        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0., 0., 0., 0.], 1.)
                .with_pass(DrawFlat::<PosTex>::new().with_transparency(
                    ColorMask::all(),
                    ALPHA,
                    None,
                ))
                .with_pass(DrawUi::new()),
        );

        // `InputBundle` provides `InputHandler<A, B>`, needed by the `UiBundle` for mouse events.
        // `UiBundle` registers `Loader<FontAsset>`, needed by `ApplicationUiBundle`.
        app_builder = app_builder
            // Provides sprite animation
            .with_bundle(AnimationBundle::<u32, Material>::new(
                "animation_control_system",
                "sampler_interpolation_system",
            ))?
            // Handles transformations of textures
            .with_bundle(
                TransformBundle::new()
                    .with_dep(&["animation_control_system", "sampler_interpolation_system"]),
            )?
            .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
            .with_bundle(InputBundle::<PlayerAxisControl, PlayerActionControl>::new())?
            .with_bundle(UiBundle::<PlayerAxisControl, PlayerActionControl>::new())?;
    }

    let mut app = app_builder.build()?;

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
