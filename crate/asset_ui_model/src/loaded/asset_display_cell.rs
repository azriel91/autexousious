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

/// Display cell for a particular asset.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct AssetDisplayCell {
    /// ID of the asset to display.
    pub asset_id: AssetId,
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
}

impl<'s> ItemComponent<'s> for AssetDisplayCell {
    type SystemData = AssetDisplayCellSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AssetDisplayCellSystemData {
            asset_display_cells,
            parent_entities,
            groundings,
            spawn_game_object_resources,
        } = system_data;

        if !asset_display_cells.contains(entity) {
            asset_display_cells
                .insert(entity, *self)
                .expect("Failed to insert `AssetDisplayCell` component.");

            // TODO: Look up sequence ID for default sequence ID for the asset type.
            let sequence_id = SequenceId::new(0);

            let spawn = Spawn {
                object: self.asset_id,
                position: Position::default(),
                velocity: Velocity::default(),
                sequence_id,
            };
            let entity_spawned =
                GameObjectSpawner::spawn(spawn_game_object_resources, entity, &spawn);

            parent_entities
                .insert(entity_spawned, ParentEntity::new(entity))
                .expect("Failed to insert `ParentEntity` component.");
            groundings
                .insert(entity_spawned, Grounding::OnGround)
                .expect("Failed to insert `Grounding` component.");
        }
    }
}
