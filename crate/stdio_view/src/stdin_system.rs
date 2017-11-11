use std::io::Write;

use amethyst::ecs::{FetchMut, RunNow};
use amethyst::shred::{Resources, SystemData};
use amethyst::shrev::EventChannel;
use application_input::ApplicationEvent;
use console::style;
use console::Term;
use dialoguer::Input;

/// Type to fetch the application event channel.
type EventChannelData<'a> = FetchMut<'a, EventChannel<ApplicationEvent>>;

/// Rendering system.
#[derive(Debug)]
pub struct StdinSystem;

impl<'a> RunNow<'a> for StdinSystem {
    fn run_now(&mut self, res: &'a Resources) {
        let mut event_channel = EventChannelData::fetch(res, 0);
        let mut term = Term::stdout();

        // TODO run Input in a separate thread. Systems are not meant to block.

        let prompt = format!("{}", style("Enter name").blue().bold());
        let input = Input::new(&prompt).interact_on(&term).unwrap();

        if let "exit" = input.as_str() {
            event_channel.single_write(ApplicationEvent::Exit);
        } else {
            writeln!(term, "Hello {}!", input).unwrap();
        }
    }
}
