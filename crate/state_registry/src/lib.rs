#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides data types to manage information about the active `State`.
//!
//! The `StateId` enum indicates which `State` is active.
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

pub use crate::{
    state_id::StateId, state_id_update_event::StateIdUpdateEvent,
    state_item_entities::StateItemEntities,
};

mod state_id;
mod state_id_update_event;
mod state_item_entities;
