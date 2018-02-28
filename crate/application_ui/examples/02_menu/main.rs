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
extern crate application_ui;
#[macro_use]
extern crate log;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod menu;
mod other;

use std::process;

use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, Pipeline, RenderBundle, Stage};
use amethyst::ui::{DrawUi, UiBundle};
use application::resource::dir;
use application::resource::find_in;
use application_ui::ApplicationUiBundle;
use structopt::StructOpt;

use menu::main_menu;
use menu::MenuBundle;

const TITLE: &str = "Example 02: Menu";

#[derive(StructOpt, Debug)]
#[structopt(name = "Example 02: Menu")]
struct Opt {
    #[structopt(long = "no-run", help = "Don't run the Amethyst application")]
    no_run: bool,
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

    let mut app = Application::build("assets", main_menu::State::new())?
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
        .with_bundle(ApplicationUiBundle::new())?
        .with_bundle(MenuBundle)?
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
