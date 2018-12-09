//! Draws text using loaded fonts.
//!
//! This example uses the following resources and assets:
//!
//! * `resources/font_config.ron`
//! * `resources/example_display_config.ron`
//! * `assets/font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-Bold.ttf`
//! * `assets/font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-BoldIt.ttf`
//! * `assets/font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-It.ttf`
//! * `assets/font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-Regular.ttf`

#[macro_use]
extern crate log;

mod main_menu;
mod other;

use std::{cell::RefCell, process, rc::Rc, time::Duration};

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, Pipeline, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
};
use application::{
    development_base_dirs,
    resource::{dir, find_in},
};
use application_robot::{
    state::{FixedTimeoutIntercept, Intercept},
    RobotState,
};
use structopt::StructOpt;

use crate::main_menu::MainMenuState;

const TITLE: &str = "Example 01: Menu";

#[derive(StructOpt, Debug)]
#[structopt(name = "Example 01: Menu")]
struct Opt {
    #[structopt(
        short = "n",
        long = "no-run",
        help = "Don't run the Amethyst application"
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

    let mut display_config = DisplayConfig::load(
        find_in(
            dir::RESOURCES,
            "display_config.ron",
            Some(development_base_dirs!()),
        )
        .unwrap(),
    );
    display_config.title = TITLE.to_string();

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.], 1.)
            .with_pass(DrawUi::new()),
    );

    let intercepts: Vec<Rc<RefCell<dyn Intercept<GameData<'_, '_>, StateEvent>>>> = {
        if let Some(timeout) = opt.timeout {
            vec![Rc::new(RefCell::new(FixedTimeoutIntercept::new(
                Duration::from_millis(timeout),
            )))]
        } else {
            vec![]
        }
    };
    let state = RobotState::new_with_intercepts(Box::new(MainMenuState::new()), intercepts);

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
