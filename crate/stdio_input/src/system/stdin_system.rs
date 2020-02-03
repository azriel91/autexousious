use std::{
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
};

use amethyst::{
    ecs::{ReadExpect, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use application_input::ApplicationEvent;
use derivative::Derivative;
use log::{debug, error, info, trace, warn};
use state_registry::StateId;
use stdio_command_model::StdinCommandBarrier;
use stdio_spi::VariantAndTokens;

use crate::{
    reader::{self, StdinReader},
    IoAppEventUtils, StatementSplitter, StatementVariant,
};

/// `StdinSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StdinSystemData<'s> {
    /// `StateId` resource.
    #[derivative(Debug = "ignore")]
    pub state_id: Option<ReadExpect<'s, StateId>>,
    /// `StdinCommandBarrier` resource.
    #[derivative(Debug = "ignore")]
    pub stdin_command_barrier: Write<'s, StdinCommandBarrier>,
    /// `ApplicationEvent` channel.
    #[derivative(Debug = "ignore")]
    pub application_ec: Write<'s, EventChannel<ApplicationEvent>>,
    /// `VariantAndTokens` channel.
    #[derivative(Debug = "ignore")]
    pub variant_and_tokens_ec: Write<'s, EventChannel<VariantAndTokens>>,
}

/// Rendering system.
#[derive(Debug)]
pub struct StdinSystem {
    /// Channel receiver for output/input messages for this system.
    rx: Receiver<String>,
}

impl StdinSystem {
    /// Returns a new `StdinSystem` that listens to stdin on a separate thread.
    // kcov-ignore-start
    pub fn new() -> Self {
        // kcov-ignore-end
        Self::default()
    }

    /// Returns a new `StdinSystem`. Visible for testing.
    ///
    /// Allows tests to retain control over the channel sender.
    pub fn internal_new<F>(rx: Receiver<String>, reader_spawn_fn: F) -> Self
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
        StdinSystemData {
            state_id,
            mut stdin_command_barrier,
            mut application_ec,
            mut variant_and_tokens_ec,
        }: Self::SystemData,
    ) {
        // Get an `Option<StateId>` from `Option<Read<StateId>>`.
        let state_id = state_id.as_deref().copied();
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
                    application_ec.single_write(ApplicationEvent::Exit);
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
                                    variant_and_tokens_ec.single_write(variant_and_tokens);
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
