#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Provides types to reference asset configuration and entities.
//!
//! This crate contains the types necessary to reference assets configuration from the file system.
//! It does not contain the types that represent actual configuration. Those are provided by the
//! respective configuration crates.
//!
//! For example, this crate contains the [`AssetIndex`][asset_index] type, which stores where object
//! configuration is, but does not contain `ObjectType` or types for the various object types.
//!
//! This crate also does not provide the logic to discover the configuration on disk. That is
//! provided by the `asset_loading` crate.
//!
//! [asset_index]: config/struct.AssetIndex.html
//!
//! # Examples
//!
//! ## Game Entities
//!
//! ```rust,ignore
//! extern crate game_model;
//!
//! use game_model::play::GameEntities;
//!
//! // Game setup state
//! fn update(&mut self, data: StateData<GameData>) -> Trans<GameData, E> {
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
//! fn update(&mut self, data: StateData<GameData>) -> Trans<GameData, E> {
//!     let game_entities = data.world.read_resource::<GameEntities>();
//!
//!     // ...
//! }
//! ```
//!


#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate derive_new;


use strum;
#[macro_use]
extern crate strum_macros;


pub mod config;
pub mod loaded;
pub mod play;
