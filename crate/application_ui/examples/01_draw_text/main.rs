#![windows_subsystem = "windows"]

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

mod render_graph;
mod state;

use std::{cell::RefCell, process, rc::Rc, time::Duration};

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{types::DefaultBackend, RenderingSystem},
    ui::UiBundle,
    window::{DisplayConfig, WindowBundle},
    GameData, StateEvent,
};
use application::{
    development_base_dirs,
    resource::{self, dir, load_in},
};
use application_robot::{
    state::{FixedTimeoutIntercept, Intercept},
    RobotState,
};
use structopt::StructOpt;

use crate::{render_graph::RenderGraph, state::TextState};

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

    let intercepts: Vec<Rc<RefCell<dyn Intercept<GameData<'_, '_>, StateEvent>>>> = {
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
        .with_bundle(WindowBundle::from_config(display_config))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RenderGraph::default(),
        ));
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
