#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Inventory of Amethyst `State`s.
//!
//! Instead of defining an enum with a variant for each `State`, an inventory removes the necessity
//! to update the central enum.
//!
//! # Usage
//!
//! The recommended way to register a `StateEntry` is to use the `state_inventory::submit!` macro:
//!
//! 1. Import the following crates in `Cargo.toml`:
//!
//!     ```toml
//!     [dependencies]
//!     inventory = "0.1.3"
//!     state_inventory = { path = "../state_inventory" }
//!     ```
//!
//! 2. Invoke the `state_inventory::submit!` macro outside the state definition.
//!
//!     ```rust,edition2018
//!     # use inventory;
//!     use state_inventory::StateEntry;
//!
//!     #[derive(Debug)]
//!     pub struct MyState;
//!
//!     // impl<'a, 'b> State<GameData<'a, 'b>, AppEvent> for MyState { /* .. */ }
//!
//!     state_inventory::submit!(MyState);
//!     #
//!     # fn main() {
//!     #     let entries = inventory::iter::<StateEntry>
//!     #         .into_iter()
//!     #         .collect::<Vec<&'static StateEntry>>();
//!     #
//!     #     assert_eq!(&&StateEntry::new("state_inventory", "MyState"), entries.first().unwrap());
//!     # }
//!     ```

pub use inventory;

pub use crate::state_entry::StateEntry;

mod state_entry;
mod submit;

inventory::collect!(StateEntry);
