#![windows_subsystem = "windows"]

//! Opens an empty window.

extern crate amethyst;
#[macro_use]
extern crate application;
extern crate application_input;
extern crate application_robot;
extern crate application_ui;
extern crate game_mode_menu;
extern crate stdio_view;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::process;

use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage};
use amethyst::ui::{DrawUi, UiBundle};
use application::resource::dir::{self, assets_dir};
use application::resource::find_in;
use application_input::ApplicationInputBundle;
use application_robot::RobotStateBuilder;
use application_ui::ApplicationUiBundle;
use stdio_view::StdinSystem;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Free Will")]
struct Opt {
    #[structopt(long = "headless", help = "Run headlessly (no GUI)")]
    headless: bool,
}

fn run(opt: &Opt) -> Result<(), amethyst::Error> {
    let state = RobotStateBuilder::default()
        .delegate(Box::new(game_mode_menu::State::new()))
        .build()
        .expect("Failed to build RobotState");
    let assets_dir = assets_dir(Some(development_base_dirs!()))?;

    let mut app_builder = Application::build(assets_dir, state)?
        .with_bundle(ApplicationInputBundle::new())?
        .with::<StdinSystem>(StdinSystem::new(), "StdinSystem", &[]);

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
                .clear_target([0., 0., 0., 1.], 1.)
                .with_pass(DrawFlat::<PosNormTex>::new())
                .with_pass(DrawUi::new()),
        );

        // `InputBundle` provides `InputHandler<A, B>`, needed by the `UiBundle` for mouse events.
        // `UiBundle` registers `Loader<FontAsset>`, needed by `ApplicationUiBundle`.
        app_builder = app_builder
            .with_bundle(InputBundle::<String, String>::new())?
            .with_bundle(UiBundle::<String, String>::new())?
            .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
            .with_bundle(ApplicationUiBundle::new())?
            .with_bundle(game_mode_menu::Bundle)?;
    }

    let mut app = app_builder.build().expect("Fatal error");

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
