use std::{
    ops::Deref,
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
};

use amethyst::{
    ecs::{ReadExpect, System, Write},
    shrev::EventChannel,
};
use application_input::ApplicationEvent;
use log::{debug, error, info, trace, warn};
use state_registry::StateId;
use stdio_command_model::StdinCommandBarrier;
use stdio_spi::VariantAndTokens;
use typename_derive::TypeName;

use crate::{
    reader::{self, StdinReader},
    IoAppEventUtils, StatementSplitter, StatementVariant,
};

/// Type to fetch the application event channel.
type StdinSystemData<'s> = (
    Option<ReadExpect<'s, StateId>>,
    Write<'s, StdinCommandBarrier>,
    Write<'s, EventChannel<ApplicationEvent>>,
    Write<'s, EventChannel<VariantAndTokens>>,
);

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

impl<'s> System<'s> for StdinSystem {
    type SystemData = StdinSystemData<'s>;

    fn run(
        &mut self,
        (state_id, mut stdin_command_barrier, mut application_event_channel, mut variant_channel): Self::SystemData,
    ) {
        // Get an `Option<StateId>` from `Option<Read<StateId>>`.
        let state_id = state_id.as_ref().map(Deref::deref).copied();
        if let Some(state_id) = state_id {
            let state_id = state_id;
            if let Some(state_id_barrier) = (*stdin_command_barrier).state_id {
                if state_id == state_id_barrier {
                    debug!("State `{:?}` running, removing `StateIdBarrier`.", state_id);

                    // Reset to `None` because we have reached this barrier.
                    (*stdin_command_barrier).state_id = None;
                } else {
                    debug!(
                        "Current state: `{:?}`, waiting for `{:?}`.",
                        state_id, state_id_barrier
                    );

                    // Skip sending events.
                    return;
                }
            };
        } else {
            warn!("`StateId` resource is not set.");
        }

        match self.rx.try_recv() {
            Ok(command_chain) => {
                debug!("`command_chain` from StdinReader: `{:?}`.", &command_chain);

                if command_chain == StdinReader::EXIT_PHRASE {
                    application_event_channel.single_write(ApplicationEvent::Exit);
                    return;
                }

                // TODO: Proper command for this.
                if command_chain == "current_state" {
                    if let Some(state_id) = state_id {
                        info!("StateId: {}", state_id);
                        return;
                    }
                }

                let statements = StatementSplitter::new(&command_chain).collect::<Vec<_>>();
                statements
                    .into_iter()
                    .filter_map(|statement| match statement {
                        Ok(StatementVariant::Default(command))
                        | Ok(StatementVariant::And(command))
                        | Ok(StatementVariant::Or(command)) => Some(command),
                        Err(statement_error) => {
                            error!("{}", statement_error);
                            None
                        }
                    })
                    .for_each(|command| {
                        match IoAppEventUtils::input_to_variant_and_tokens(&command) {
                            Ok(variant_and_tokens) => {
                                if let Some(variant_and_tokens) = variant_and_tokens {
                                    variant_channel.single_write(variant_and_tokens);
                                }
                            }
                            Err(e) => error!("Failed to parse command. Error: `{}`.", e),
                        }
                    });
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
        shred::{SystemData, World},
        shrev::{EventChannel, ReaderId},
    };
    use application_event::AppEventVariant;
    use application_input::ApplicationEvent;
    use state_registry::StateId;
    use stdio_command_model::StdinCommandBarrier;
    use stdio_spi::VariantAndTokens;

    use super::{StdinSystem, StdinSystemData};

    fn setup() -> (
        StdinSystem,
        Sender<String>,
        World,
        ReaderId<ApplicationEvent>,
        ReaderId<VariantAndTokens>,
    ) {
        setup_with_barrier(None)
    }

    fn setup_with_barrier(
        with_barrier: Option<bool>,
    ) -> (
        StdinSystem,
        Sender<String>,
        World,
        ReaderId<ApplicationEvent>,
        ReaderId<VariantAndTokens>,
    ) {
        let mut world = World::empty();
        world.insert(StateId::CharacterSelection);
        let barrier_state_id = with_barrier.map(|barrier_matches| {
            if barrier_matches {
                StateId::CharacterSelection
            } else {
                StateId::Loading
            }
        });
        let stdin_command_barrier = StdinCommandBarrier::new(barrier_state_id);
        world.insert(stdin_command_barrier);
        world.insert(EventChannel::<ApplicationEvent>::with_capacity(10));
        world.insert(EventChannel::<VariantAndTokens>::with_capacity(10));

        let (tx, rx) = mpsc::channel();
        let stdin_system = StdinSystem::internal_new(rx, || {});

        let (application_ev_id, variant_and_tokens_id) = {
            let (_, _, mut application_events, mut variant_and_tokens) =
                StdinSystemData::fetch(&world);
            (
                application_events.register_reader(),
                variant_and_tokens.register_reader(),
            ) // kcov-ignore
        }; // kcov-ignore

        (
            stdin_system,
            tx,
            world,
            application_ev_id,
            variant_and_tokens_id,
        )
    }

    #[test]
    fn sends_exit_event_when_input_is_exit() {
        let (mut stdin_system, tx, world, mut application_ev_id, _) = setup();

        tx.send("exit".to_string()).unwrap();
        stdin_system.run_now(&world);

        let (_, _, application_events, _) = StdinSystemData::fetch(&world);

        expect_event(
            &application_events,
            &mut application_ev_id,
            Some(&ApplicationEvent::Exit),
        );
    } // kcov-ignore

    #[test]
    fn does_not_send_exit_event_when_input_is_not_exit() {
        let (mut stdin_system, tx, world, mut application_ev_id, _) = setup();

        tx.send("abc".to_string()).unwrap();
        stdin_system.run_now(&world);

        let (_, _, application_events, _) = StdinSystemData::fetch(&world);
        expect_event(&application_events, &mut application_ev_id, None);
    } // kcov-ignore

    #[test]
    fn does_nothing_when_input_is_empty() {
        let (mut stdin_system, _tx, world, mut application_ev_id, _) = setup();

        // we don't call tx.send(..)
        stdin_system.run_now(&world);

        let (_, _, application_events, _) = StdinSystemData::fetch(&world);
        expect_event(&application_events, &mut application_ev_id, None);
    } // kcov-ignore

    #[test]
    fn does_not_panic_when_application_channel_is_disconnected() {
        let (mut stdin_system, tx, world, mut application_ev_id, _) = setup();

        drop(tx); // ensure channel is disconnected
        stdin_system.run_now(&world);

        let (_, _, application_events, _) = StdinSystemData::fetch(&world);
        expect_event(&application_events, &mut application_ev_id, None);
    } // kcov-ignore

    #[test]
    fn sends_vat_event_when_input_is_app_event() {
        let (mut stdin_system, tx, world, _, mut vat_ev_id) = setup();

        tx.send("character_selection confirm".to_string()).unwrap();
        stdin_system.run_now(&world);

        let (_, _, _, vat_events) = StdinSystemData::fetch(&world);

        expect_vat_event(
            &vat_events,
            &mut vat_ev_id,
            Some(&(
                AppEventVariant::CharacterSelection,
                vec!["character_selection".to_string(), "confirm".to_string()],
            )),
        ); // kcov-ignore
    }

    #[test]
    fn does_not_send_exit_event_when_barrier_does_not_match() {
        let (mut stdin_system, tx, world, mut application_ev_id, _) =
            setup_with_barrier(Some(false));

        tx.send("exit".to_string()).unwrap();
        stdin_system.run_now(&world);

        let (_, _, application_events, _) = StdinSystemData::fetch(&world);

        expect_event(&application_events, &mut application_ev_id, None);
    }

    #[test]
    fn sends_exit_event_when_barrier_matches() {
        let (mut stdin_system, tx, world, mut application_ev_id, _) =
            setup_with_barrier(Some(true));

        tx.send("exit".to_string()).unwrap();
        stdin_system.run_now(&world);

        let (_, _, application_events, _) = StdinSystemData::fetch(&world);

        expect_event(
            &application_events,
            &mut application_ev_id,
            Some(&ApplicationEvent::Exit),
        );
    }

    #[test]
    fn does_not_send_vat_event_when_barrier_does_not_match() {
        let (mut stdin_system, tx, world, _, mut vat_ev_id) = setup_with_barrier(Some(false));

        tx.send("character_selection confirm".to_string()).unwrap();
        stdin_system.run_now(&world);

        let (_, _, _, vat_events) = StdinSystemData::fetch(&world);

        expect_vat_event(&vat_events, &mut vat_ev_id, None); // kcov-ignore
    }

    #[test]
    fn sends_vat_event_when_barrier_matches() {
        let (mut stdin_system, tx, world, _, mut vat_ev_id) = setup_with_barrier(Some(true));

        tx.send("character_selection confirm".to_string()).unwrap();
        stdin_system.run_now(&world);

        let (_, _, _, vat_events) = StdinSystemData::fetch(&world);

        expect_vat_event(
            &vat_events,
            &mut vat_ev_id,
            Some(&(
                AppEventVariant::CharacterSelection,
                vec!["character_selection".to_string(), "confirm".to_string()],
            )),
        ); // kcov-ignore
    }

    fn expect_event(
        application_events: &EventChannel<ApplicationEvent>,
        mut application_ev_id: &mut ReaderId<ApplicationEvent>,
        expected_event: Option<&ApplicationEvent>,
    ) {
        let mut event_it = application_events.read(&mut application_ev_id);
        assert_eq!(expected_event, event_it.next());
    }

    fn expect_vat_event(
        vat_events: &EventChannel<VariantAndTokens>,
        mut vat_ev_id: &mut ReaderId<VariantAndTokens>,
        expected_event: Option<&VariantAndTokens>,
    ) {
        let mut event_it = vat_events.read(&mut vat_ev_id);
        assert_eq!(expected_event, event_it.next());
    }
}
