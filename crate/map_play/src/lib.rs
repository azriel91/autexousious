#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides systems that update the map during game play.

pub use crate::{
    comparative::Comparative,
    map_bounds_checks::MapBoundsChecks,
    map_spawner::MapSpawner,
    map_spawner_resources::MapSpawnerResources,
    system::{
        KeepWithinMapBoundsSystem, MapEnterExitDetectionSystem, MapOutOfBoundsClockAugmentSystem,
        MapOutOfBoundsDeletionSystem, MapSpawnOutOfBoundsDetectionSystem,
        OUT_OF_BOUNDS_DELETE_DELAY,
    },
};

mod comparative;
mod map_bounds_checks;
mod map_spawner;
mod map_spawner_resources;
mod system;
