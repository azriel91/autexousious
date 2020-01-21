use std::convert::TryFrom;

use amethyst::{
    ecs::{storage::VecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{config::AssetType, loaded::AssetId, ItemComponent};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::{
    config::{Position, Velocity},
    play::PositionInitParent,
};
use map_play::{MapSpawner, MapSpawnerResources};
use object_model::play::Grounding;
use object_type::ObjectType;
use parent_model::play::ParentEntity;
use sequence_model::loaded::SequenceId;
use spawn_model::loaded::Spawn;
use spawn_play::{GameObjectSpawner, SpawnGameObjectResources};

use crate::config::Dimensions;

/// Display cell for a particular asset.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct AssetDisplayCell {
    /// ID of the asset to display.
    pub asset_id: AssetId,
    /// Type of the asset.
    ///
    /// This is used to determine how to display the asset.
    pub asset_type: AssetType,
    /// Size of the cell.
    ///
    /// This is used to position the spawned asset.
    pub cell_size: Dimensions,
}

impl AssetDisplayCell {
    fn spawn_character(
        &self,
        AssetDisplayCellSystemData {
            parent_entities,
            groundings,
            spawn_game_object_resources,
            ..
        }: &mut AssetDisplayCellSystemData,
        entity: Entity,
    ) {
        // TODO: Look up sequence ID for default sequence ID for the asset type.
        let sequence_id = SequenceId::new(0);
        let half_cell_width = i16::try_from(self.cell_size.w >> 1)
            .expect("Failed to convert `cell_size.w` to `i16`.");
        let half_cell_width = f32::from(half_cell_width);

        let spawn = Spawn {
            object: self.asset_id,
            position: Position::new(half_cell_width, 10., 0.),
            velocity: Velocity::default(),
            sequence_id,
        };
        let entity_spawned = GameObjectSpawner::spawn(spawn_game_object_resources, entity, &spawn);

        parent_entities
            .insert(entity_spawned, ParentEntity::new(entity))
            .expect("Failed to insert `ParentEntity` component.");
        groundings
            .insert(entity_spawned, Grounding::OnGround)
            .expect("Failed to insert `Grounding` component.");
    }

    fn spawn_map(
        &self,
        AssetDisplayCellSystemData {
            parent_entities,
            position_init_parents,
            map_spawner_resources,
            ..
        }: &mut AssetDisplayCellSystemData,
        entity: Entity,
    ) {
        // TODO: scale map down
        // let cell_width = self.cell_size.w as f32;

        let map_entities = MapSpawner::spawn(map_spawner_resources, self.asset_id);
        let parent_entity = ParentEntity::new(entity);
        map_entities.iter().copied().for_each(|map_entity| {
            parent_entities
                .insert(map_entity, parent_entity)
                .expect("Failed to insert `ParentEntity` component.");
            position_init_parents
                .insert(map_entity, PositionInitParent::new(entity))
                .expect("Failed to insert `PositionInitParent` component.");
        });
    }
}

/// `AssetDisplayCellSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetDisplayCellSystemData<'s> {
    /// `AssetDisplayCell` components.
    #[derivative(Debug = "ignore")]
    pub asset_display_cells: WriteStorage<'s, AssetDisplayCell>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
    /// `SpawnGameObjectResources`.
    pub spawn_game_object_resources: SpawnGameObjectResources<'s>,
    /// `PositionInitParent` components.
    #[derivative(Debug = "ignore")]
    pub position_init_parents: WriteStorage<'s, PositionInitParent>,
    /// `MapSpawnerResources`.
    pub map_spawner_resources: MapSpawnerResources<'s>,
}

impl<'s> ItemComponent<'s> for AssetDisplayCell {
    type SystemData = AssetDisplayCellSystemData<'s>;

    fn augment(&self, mut asset_display_cell_system_data: &mut Self::SystemData, entity: Entity) {
        if !asset_display_cell_system_data
            .asset_display_cells
            .contains(entity)
        {
            asset_display_cell_system_data
                .asset_display_cells
                .insert(entity, *self)
                .expect("Failed to insert `AssetDisplayCell` component.");

            match self.asset_type {
                AssetType::Object(ObjectType::Character) => {
                    self.spawn_character(&mut asset_display_cell_system_data, entity);
                }
                AssetType::Map => {
                    self.spawn_map(&mut asset_display_cell_system_data, entity);
                }
                asset_type => {
                    log::error!(
                        "`AssetDisplayCell` does not support displaying `{:?}`.",
                        asset_type
                    );
                }
            }
        }
    }
}
