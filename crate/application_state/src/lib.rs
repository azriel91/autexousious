#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `AutexState` trait to simplify implementing `amethyst::State`.

#[macro_use]
extern crate derive_deref;
#[macro_use]
extern crate log;

pub use crate::{
    app_state::{AppState, AppStateBuilder},
    autex_state::AutexState,
    hook_fn::HookFn,
    hookable_fn::HookableFn,
};

mod app_state;
mod autex_state;
mod hook_fn;
mod hookable_fn;
