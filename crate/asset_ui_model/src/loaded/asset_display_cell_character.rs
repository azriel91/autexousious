use std::convert::TryFrom;

use amethyst::{
    ecs::{storage::VecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::AssetId, ItemComponent};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::{Position, Velocity};
use object_model::play::Grounding;
use parent_model::play::ParentEntity;
use sequence_model::loaded::SequenceId;
use spawn_model::loaded::Spawn;
use spawn_play::{GameObjectSpawner, SpawnGameObjectResources};

use crate::config::Dimensions;

/// Display cell for a character asset.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct AssetDisplayCellCharacter {
    /// ID of the asset to display.
    pub asset_id: AssetId,
    /// Size of the cell.
    ///
    /// This is used to position the spawned asset.
    pub cell_size: Dimensions,
}

impl AsRef<AssetId> for AssetDisplayCellCharacter {
    fn as_ref(&self) -> &AssetId {
        &self.asset_id
    }
}

impl AssetDisplayCellCharacter {
    fn spawn_character(
        &self,
        AssetDisplayCellCharacterSystemData {
            parent_entities,
            groundings,
            spawn_game_object_resources,
            ..
        }: &mut AssetDisplayCellCharacterSystemData,
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
}

/// `AssetDisplayCellCharacterSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetDisplayCellCharacterSystemData<'s> {
    /// `AssetDisplayCellCharacter` components.
    #[derivative(Debug = "ignore")]
    pub asset_display_cells: WriteStorage<'s, AssetDisplayCellCharacter>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
    /// `SpawnGameObjectResources`.
    pub spawn_game_object_resources: SpawnGameObjectResources<'s>,
}

impl<'s> ItemComponent<'s> for AssetDisplayCellCharacter {
    type SystemData = AssetDisplayCellCharacterSystemData<'s>;

    fn augment(&self, mut asset_display_cell_system_data: &mut Self::SystemData, entity: Entity) {
        if !asset_display_cell_system_data
            .asset_display_cells
            .contains(entity)
        {
            asset_display_cell_system_data
                .asset_display_cells
                .insert(entity, *self)
                .expect("Failed to insert `AssetDisplayCellCharacter` component.");

            self.spawn_character(&mut asset_display_cell_system_data, entity);
        }
    }
}
