#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides types to reference loaded game object and map models.
//!
//! # Examples
//!
//! ## Game Entities
//!
//! ```rust,ignore
//! extern crate asset_model;
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
//!     data.world.insert(game_entities);
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

pub mod loaded;
pub mod play;
