//! Opens an empty window.

extern crate amethyst;
#[macro_use]
extern crate application;
extern crate stdio_view;

use std::process;

use amethyst::renderer::{DisplayConfig, DrawFlat, Event, KeyboardInput, Pipeline, PosNormTex,
                         RenderBundle, RenderSystem, Stage, VirtualKeyCode, WindowEvent};
use amethyst::prelude::*;
use application::config::find_in;
use stdio_view::StdinSystem;

struct Example;

impl State for Example {
    fn handle_event(&mut self, _: &mut Engine, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } |
                WindowEvent::Closed => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }
}

fn run() -> Result<(), amethyst::Error> {
    let display_config = DisplayConfig::load(
        find_in(
            "resources",
            "display_config.ron",
            Some(development_base_dirs!()),
        ).unwrap()
            .as_os_str()
            .to_str()
            .unwrap(),
    );

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.2, 0.4, 1.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let mut app = Application::build(".", Example)?
        .with_bundle(RenderBundle::new())?
        .with_local(RenderSystem::build(pipe, Some(display_config))?)
        .with_local(StdinSystem)
        .build()
        .expect("Fatal error");

    app.run();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Failed to execute example: {}", e);
        process::exit(1);
    }
}
