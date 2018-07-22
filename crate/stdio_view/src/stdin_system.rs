use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

use amethyst::{
    ecs::prelude::{System, Write},
    shrev::EventChannel,
};
use application_input::ApplicationEvent;

use reader::{self, StdinReader};

/// Type to fetch the application event channel.
type EventChannelData<'a> = Write<'a, EventChannel<ApplicationEvent>>;

/// Rendering system.
#[derive(Debug, TypeName)]
pub struct StdinSystem {
    /// Channel receiver for output/input messages for this system.
    rx: Receiver<String>,
}

impl StdinSystem {
    /// Returns a new StdinSystem that listens to stdin on a separate thread.
    // kcov-ignore-start
    pub fn new() -> Self {
        // kcov-ignore-end
        Self::default()
    }

    /// Returns a new StdinSystem
    ///
    /// Allows tests to retain control over the channel sender.
    fn internal_new<F>(rx: Receiver<String>, reader_spawn_fn: F) -> Self
    where
        F: FnOnce(),
    {
        reader_spawn_fn();
        StdinSystem { rx }
    }
}

impl Default for StdinSystem {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        let reader_spawn_fn = || {
            thread::Builder::new()
                .name(reader::NAME.to_string())
                .spawn(|| StdinReader::new(tx).start())
                // TODO: replace new() with build() and return Result<..>
                .expect("Failed to spawn StdinReader thread.");
        };
        Self::internal_new(rx, reader_spawn_fn)
    } // kcov-ignore
}

impl<'a> System<'a> for StdinSystem {
    type SystemData = EventChannelData<'a>;

    fn run(&mut self, mut event_channel: Self::SystemData) {
        match self.rx.try_recv() {
            Ok(msg) => {
                debug!("Received message from StdinReader: \"{}\".", msg);
                if let "exit" = msg.as_str() {
                    event_channel.single_write(ApplicationEvent::Exit);
                }
            }
            Err(TryRecvError::Empty) => {
                // do nothing
                trace!("No message from StdinReader");
            }
            Err(TryRecvError::Disconnected) => {
                warn!("Channel receiver to `StdinReader` disconnected.");
            }
        };
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::{self, Sender};

    use amethyst::{
        ecs::prelude::RunNow,
        shred::{Resources, SystemData},
        shrev::{EventChannel, ReaderId},
    };
    use application_input::ApplicationEvent;

    use super::{EventChannelData, StdinSystem};

    fn setup() -> (
        StdinSystem,
        Sender<String>,
        Resources,
        ReaderId<ApplicationEvent>,
    ) {
        let mut res = Resources::new();
        res.insert(EventChannel::<ApplicationEvent>::with_capacity(10));
        let (tx, rx) = mpsc::channel();
        let stdin_system = StdinSystem::internal_new(rx, || {});

        let reader_id = {
            let mut event_channel = EventChannelData::fetch(&res);
            event_channel.register_reader()
        }; // kcov-ignore

        (stdin_system, tx, res, reader_id)
    }

    fn expect_event(
        event_channel: &EventChannel<ApplicationEvent>,
        mut reader_id: &mut ReaderId<ApplicationEvent>,
        expected_event: Option<&ApplicationEvent>,
    ) {
        let mut event_it = event_channel.read(&mut reader_id);
        assert_eq!(expected_event, event_it.next());
    }

    #[test]
    fn sends_exit_event_when_input_is_exit() {
        let (mut stdin_system, tx, res, mut reader_id) = setup();

        tx.send("exit".to_string()).unwrap();
        stdin_system.run_now(&res);

        let event_channel = EventChannelData::fetch(&res);

        expect_event(
            &event_channel,
            &mut reader_id,
            Some(&ApplicationEvent::Exit),
        );
    } // kcov-ignore

    #[test]
    fn does_not_send_exit_event_when_input_is_not_exit() {
        let (mut stdin_system, tx, res, mut reader_id) = setup();

        tx.send("abc".to_string()).unwrap();
        stdin_system.run_now(&res);

        let event_channel = EventChannelData::fetch(&res);
        expect_event(&event_channel, &mut reader_id, None);
    } // kcov-ignore

    #[test]
    fn does_nothing_when_input_is_empty() {
        let (mut stdin_system, _tx, res, mut reader_id) = setup();

        // we don't call tx.send(..)
        stdin_system.run_now(&res);

        let event_channel = EventChannelData::fetch(&res);
        expect_event(&event_channel, &mut reader_id, None);
    } // kcov-ignore

    #[test]
    fn does_not_panic_when_channel_is_disconnected() {
        let (mut stdin_system, tx, res, mut reader_id) = setup();

        drop(tx); // ensure channel is disconnected
        stdin_system.run_now(&res);

        let event_channel = EventChannelData::fetch(&res);
        expect_event(&event_channel, &mut reader_id, None);
    } // kcov-ignore
}
