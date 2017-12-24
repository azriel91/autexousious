//! Opens an empty window.

extern crate amethyst;
#[macro_use]
extern crate application;
extern crate application_input;
extern crate stdio_view;

use std::process;

use amethyst::renderer::{DisplayConfig, DrawFlat, Event, KeyboardInput, Pipeline, PosNormTex,
                         RenderBundle, RenderSystem, Stage, VirtualKeyCode, WindowEvent};
use amethyst::prelude::*;
use amethyst::shrev::{EventChannel, EventReadData, ReaderId};
use application::config::find_in;
use application_input::{ApplicationEvent, ApplicationInputBundle};
use stdio_view::StdinSystem;

#[derive(Debug, Default)]
struct Example {
    reader: Option<ReaderId>,
}

impl Example {
    pub fn new() -> Self {
        Default::default()
    }
}

impl State for Example {
    fn on_start(&mut self, world: &mut World) {
        // You can't unregister a reader from an EventChannel in on_stop because we don't have to
        //
        // @torkleyy: No need to unregister, it's just two integer values.
        // @Rhuagh: Just drop the reader id
        let reader_id = world
            .read_resource::<EventChannel<ApplicationEvent>>()
            .register_reader();

        self.reader.get_or_insert(reader_id);
    }

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }
                | WindowEvent::Closed => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn update(&mut self, world: &mut World) -> Trans {
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();

        let mut reader_id = self.reader.as_mut().expect("Expected reader to be set");
        if let Ok(event_read_data) = app_event_channel.read(&mut reader_id) {
            match event_read_data {
                EventReadData::Data(mut storage_iterator) => {
                    while let Some(_event) = storage_iterator.next() {
                        return Trans::Quit;
                    }
                }
                _ => {}
            }
        } else {
            unimplemented!();
        }

        Trans::None
    }
}

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
            .clear_target([0.2, 0.4, 1.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let mut app = Application::build(".", Example::new())?
        .with_bundle(RenderBundle::new())?
        .with_bundle(ApplicationInputBundle::new())?
        .with_local(RenderSystem::build(pipe, Some(display_config))?)
        .with_local(StdinSystem::new())
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
