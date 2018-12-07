//! Draws text using loaded fonts.
//!
//! This example uses the following resources and assets:
//!
//! * `resources/font_config.ron`
//! * `resources/display_config.ron`
//! * `assets/font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-Bold.ttf`
//! * `assets/font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-BoldIt.ttf`
//! * `assets/font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-It.ttf`
//! * `assets/font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-Regular.ttf`

extern crate amethyst;
#[macro_use]
extern crate application;
extern crate application_robot;
extern crate application_ui;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod state;

use std::cell::RefCell;
use std::process;
use std::rc::Rc;
use std::time::Duration;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, Pipeline, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
};
use application::resource::{self, dir, load_in};
use application_robot::{
    state::{FixedTimeoutIntercept, Intercept},
    RobotState,
};
use structopt::StructOpt;

use crate::state::TextState;

#[derive(StructOpt, Debug)]
#[structopt(name = "Example 01: Draw Text")]
struct Opt {
    #[structopt(
        long = "no-run",
        help = "Initialize, but don't run the Amethyst application"
    )]
    no_run: bool,
    #[structopt(
        short = "t",
        long = "timeout",
        help = "Timeout to automatically close the application"
    )]
    timeout: Option<u64>,
}

fn run(opt: &Opt) -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let display_config = load_in::<DisplayConfig, _>(
        dir::RESOURCES,
        "display_config.ron",
        resource::Format::Ron,
        Some(development_base_dirs!()),
    )?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.], 1.)
            .with_pass(DrawUi::new()),
    );

    let intercepts: Vec<Rc<RefCell<Intercept<GameData, StateEvent>>>> = {
        if let Some(timeout) = opt.timeout {
            vec![Rc::new(RefCell::new(FixedTimeoutIntercept::new(
                Duration::from_millis(timeout),
            )))]
        } else {
            vec![]
        }
    };
    let state = RobotState::new_with_intercepts(Box::new(TextState), intercepts);

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?;
    let mut app = Application::new("assets", state, game_data)?;

    if !opt.no_run {
        app.run();
    }

    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = run(&opt) {
        println!("Failed to execute example: {}", e);
        process::exit(1);
    }
}
