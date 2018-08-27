#![windows_subsystem = "windows"]

//! Opens an empty window.

extern crate amethyst;
extern crate application_robot;
extern crate stdio_view;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::cell::RefCell;
use std::process;
use std::rc::Rc;
use std::time::Duration;

use amethyst::prelude::*;
use application_robot::{state::FixedTimeoutIntercept, RobotState};
use stdio_view::StdioViewBundle;
use structopt::StructOpt;

#[derive(Debug)]
struct EmptyState;

#[derive(StructOpt, Debug)]
#[structopt(name = "Example 01: Read and Exit")]
struct Opt {
    #[structopt(
        short = "t",
        long = "timeout",
        help = "Timeout to automatically close the application"
    )]
    timeout: Option<u64>,
}

impl<'a, 'b> State<GameData<'a, 'b>, ()> for EmptyState {
    fn on_start(&mut self, _data: StateData<GameData>) {
        println!("Reading from stdin. Type 'exit' to quit.");
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, ()> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn run(opt: &Opt) -> Result<(), amethyst::Error> {
    let mut intercepts = RobotState::default_intercepts();
    if let Some(timeout) = opt.timeout {
        intercepts.push(Rc::new(RefCell::new(FixedTimeoutIntercept::new(
            Duration::from_millis(timeout),
        ))));
    }

    let state = RobotState::new_with_intercepts(Box::new(EmptyState), intercepts);
    let game_data = GameDataBuilder::default().with_bundle(StdioViewBundle::new())?;
    let assets_dir = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
    Application::new(assets_dir, state, game_data)?.run();
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(&opt) {
        println!("Failed to execute example: {}", e);
        process::exit(1);
    }
}
