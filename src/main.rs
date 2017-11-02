//! Opens an empty window.

extern crate amethyst;

use std::env;
use std::path::{Path, PathBuf};
use std::process;

use amethyst::renderer::{DisplayConfig, DrawFlat, Event, KeyboardInput, Pipeline, PosNormTex,
                         RenderBundle, RenderSystem, Stage, VirtualKeyCode, WindowEvent};
use amethyst::prelude::*;

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

fn renderer_config() -> Result<PathBuf, &'static str> {
    let mut exe_dir = env::current_exe().unwrap();
    exe_dir.pop();

    // Not sure that we need to have both OUT_DIR and CARGO_MANIFEST_DIR checked, but when we add
    // other resources we probably want to just read OUT_DIR and not CARGO_MANIFEST_DIR
    let base_dirs = vec![
        exe_dir,
        Path::new(env!("OUT_DIR")).to_owned(),
        Path::new(env!("CARGO_MANIFEST_DIR")).to_owned(),
    ];
    for base_dir in &base_dirs {
        let mut config_path = base_dir.join("resources");
        config_path.push("config.ron");

        if config_path.exists() {
            return Ok(config_path);
        }
    }

    Err("Failed to find resources/config.ron")
}

fn run() -> Result<(), amethyst::Error> {
    let config = DisplayConfig::load(renderer_config().unwrap().as_os_str().to_str().unwrap());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.2, 0.4, 1.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let mut app = Application::build(".", Example)?
        .with_bundle(RenderBundle::new())?
        .with_local(RenderSystem::build(pipe, Some(config))?)
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
