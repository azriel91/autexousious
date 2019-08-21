#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides an enum that represents `State`s.
//!
//! This crate exists to indicate which `State` is active.
//!
//! ```rust,ignore
//! use state_registry::StateId;
//!
//! impl<'a, 'b> State<GameData<'a, 'b>, AppEvent> for GamePlayState {
//!     fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
//!         data.world.insert(StateId::GamePlay);
//!     }
//! }
//! ```

pub use crate::state_id::StateId;

mod state_id;
