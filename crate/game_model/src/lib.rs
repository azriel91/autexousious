#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Provides types to manage game configuration and entities.
//!
//! This crate contains the types necessary to discover game configuration from the file system; it
//! does not contain the types that represent actual configuration. Those are provided by the
//! respective configuration crates.
//!
//! For example, this crate contains the [`ConfigIndex`][cfg_index] type, which stores where object
//! configuration is, but does not contain `ObjectType` or types for the various object types.
//!
//! [cfg_index]: config/enum.ConfigIndex.html
//!
//! # Examples
//!
//! ## Game Configuration
//!
//! ```rust
//! extern crate game_model;
//!
//! use std::path::PathBuf;
//!
//! use game_model::config;
//!
//! fn main() {
//!     let assets_dir = PathBuf::from(format!("{}/assets", env!("CARGO_MANIFEST_DIR")));
//!     let config_index = config::index_configuration(&assets_dir);
//!     println!("{:#?}", config_index);
//! }
//! ```
//!
//! ## Game Entities
//!
//! ```rust,ignore
//! extern crate game_model;
//!
//! use game_model::play::GameEntities;
//!
//! // Game setup state
//! fn update(&mut self, data: StateData<GameData>) -> Trans<GameData> {
//!     let objects = HashMap::<ObjectType, Entity>::new();
//!     let map_layers = Vec::new();
//!
//!     // Create entities and store them in the map and vec
//!
//!     let game_entities = GameEntities::new(objects, map_layers);
//!     data.world.add_resource(game_entities);
//!     Trans::Switch(Box::new())
//! }
//!
//! // ...
//!
//! // Game play state
//! fn update(&mut self, data: StateData<GameData>) -> Trans<GameData> {
//!     let game_entities = data.world.read_resource::<GameEntities>();
//!
//!     // ...
//! }
//! ```
//!

extern crate amethyst;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate derive_new;
extern crate heck;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate map_model;
extern crate object_model;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[cfg(test)]
extern crate tempfile;

pub mod config;
pub mod play;
