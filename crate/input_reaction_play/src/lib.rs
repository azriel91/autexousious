#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Logic for input reactions at runtime.

pub use crate::{
    ir_app_event_sender::IrAppEventSender,
    system::{
        ButtonInputReactionsTransitionSystem, ButtonInputReactionsTransitionSystemDesc,
        InputReactionsTransitionSystem, InteractableObjectSyncSystem,
    },
    system_data::IrAppEventSenderSystemData,
};

mod ir_app_event_sender;
mod system;
mod system_data;
