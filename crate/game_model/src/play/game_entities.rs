use std::collections::HashMap;

use amethyst::ecs::prelude::*;
use object_model::ObjectType;

/// All entities for a game.
#[derive(Clone, Debug, new)]
pub struct GameEntities {
    /// Map of object entities by object type.
    pub objects: HashMap<ObjectType, Vec<Entity>>,
    /// Map layer entities.
    pub map_layers: Vec<Entity>,
}

impl GameEntities {
    /// Returns an iterator to immutable references to all game entities.
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.objects
            .values()
            .flat_map(|entities| entities)
            .chain(self.map_layers.iter())
    }

    /// Returns a consuming iterator to all game entities.
    pub fn drain(&mut self) -> impl Iterator<Item = Entity> + '_ {
        self.objects
            .drain()
            .flat_map(|(_, entities)| entities)
            .chain(self.map_layers.drain(..))
    }
}
