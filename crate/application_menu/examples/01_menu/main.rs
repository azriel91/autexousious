#![windows_subsystem = "windows"]

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

mod main_menu;
mod other;

use std::{cell::RefCell, process, rc::Rc, time::Duration};

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{plugins::RenderToWindow, types::DefaultBackend, RenderingBundle},
    ui::{RenderUi, UiBundle},
    window::DisplayConfig,
    GameData, StateEvent,
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
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config).with_clear([0., 0., 0., 1.0]),
                )
                .with_plugin(RenderUi::default()),
        )?;
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
