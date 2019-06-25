use amethyst::ecs::Entity;
use derive_new::new;

use crate::config::Spawn;

/// Event indicating an object was just `Spawn`ed.
///
/// The base use of this is to allow the spawned entity to be rectified.
///
/// Other possible uses of this event are:
///
/// * Buffs to improve the spawned entity.
/// * Debuffs to remove the spawned entity.
#[derive(Clone, Debug, PartialEq, new)]
pub struct SpawnEvent {
    /// `Spawn` that this event pertains to.
    pub spawn: Spawn,
    /// `Entity` that created the spawned `Entity`.
    pub entity_parent: Entity,
    /// `Entity` that was spawned.
    pub entity_spawned: Entity,
}
