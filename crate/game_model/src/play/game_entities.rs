use std::collections::HashMap;

use amethyst::ecs::prelude::*;
use derive_new::new;
use object_type::ObjectType;

/// All entities for a game.
#[derive(Clone, Debug, Default, new)]
pub struct GameEntities {
    /// Map of object entities by object type.
    pub objects: HashMap<ObjectType, Vec<Entity>>,
    /// Map sprite sequence entities.
    pub map_layers: Vec<Entity>,
}

impl GameEntities {
    /// Returns an iterator to immutable references to all game entities.
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.objects
            .values()
            .flatten()
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
