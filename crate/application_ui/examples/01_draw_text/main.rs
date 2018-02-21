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

mod state;

use std::process;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, Pipeline, RenderBundle, Stage};
use amethyst::ui::{DrawUi, UiBundle};
use application::config::find_in;
use application_ui::ApplicationUiBundle;

use state::TextState;

fn run() -> Result<(), amethyst::Error> {
    let display_config = DisplayConfig::load(
        find_in(
            "resources",
            "display_config.ron",
            Some(development_base_dirs!()),
        ).unwrap(),
    );

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.], 1.)
            .with_pass(DrawUi::new()),
    );

    let mut app = Application::build("assets", TextState)?
        .with_bundle(UiBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
        .with_bundle(ApplicationUiBundle::new())?
        .build()
        .expect("Failed to build application.");

    app.run();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Failed to execute example: {}", e);
        process::exit(1);
    }
}
