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

extern crate amethyst;
#[macro_use]
extern crate application;
extern crate application_menu;
extern crate application_robot;
extern crate application_ui;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate log;
extern crate rayon;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod main_menu;
mod other;

use std::process;
use std::time::Duration;

use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, Pipeline, RenderBundle, Stage};
use amethyst::ui::{DrawUi, UiBundle};
use application::resource::dir;
use application::resource::find_in;
use application_robot::RobotStateBuilder;
use application_robot::state::FixedTimeoutIntercept;
use application_ui::ApplicationUiBundle;
use structopt::StructOpt;

const TITLE: &str = "Example 01: Menu";

#[derive(StructOpt, Debug)]
#[structopt(name = "Example 01: Menu")]
struct Opt {
    #[structopt(short = "n", long = "no-run", help = "Don't run the Amethyst application")]
    no_run: bool,
    #[structopt(short = "t", long = "timeout",
                help = "Timeout to automatically close the application")]
    timeout: Option<u64>,
}

fn run(opt: &Opt) -> Result<(), amethyst::Error> {
    let mut display_config = DisplayConfig::load(
        find_in(
            dir::RESOURCES,
            "display_config.ron",
            Some(development_base_dirs!()),
        ).unwrap(),
    );
    display_config.title = TITLE.to_string();

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.], 1.)
            .with_pass(DrawUi::new()),
    );

    let mut state = RobotStateBuilder::default()
        .delegate(Box::new(main_menu::State::new()))
        .build()
        .unwrap();

    // TODO: Use setter method, pending <https://gitlab.com/azriel91/autexousious/issues/17>
    state.intercepts = {
        if let Some(timeout) = opt.timeout {
            vec![
                Box::new(FixedTimeoutIntercept::new(Duration::from_millis(timeout))),
            ]
        } else {
            vec![]
        }
    };

    let mut app = Application::build("assets", state)?
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
        .with_bundle(ApplicationUiBundle::new())?
        .with_bundle(main_menu::Bundle)?
        .build()
        .expect("Failed to build application.");

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
