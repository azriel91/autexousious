use amethyst::{
    ecs::{Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::play::AssetSelection;
use asset_selection_ui_model::play::ApwPreview;
use asset_ui_model::play::AssetSelectionParent;
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::{Position, Velocity};
use object_model::play::Grounding;
use parent_model::play::ParentEntity;
use sequence_model::loaded::SequenceId;
use spawn_model::loaded::Spawn;
use spawn_play::{GameObjectSpawner, SpawnGameObjectResources};

use super::PreviewSpawner;

/// Spawns / deletes character preview entities when character selection is switched.
#[derive(Debug, Default, new)]
pub struct CharacterPreviewSpawn;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterPreviewSpawnResources<'s> {
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
    /// `SpawnGameObjectResources`.
    pub spawn_game_object_resources: SpawnGameObjectResources<'s>,
}

impl<'s> PreviewSpawner<'s> for CharacterPreviewSpawn {
    type SystemData = CharacterPreviewSpawnResources<'s>;

    // Spawns new entities that provide a preview for the asset preview widget.
    fn spawn_preview_entities(
        apw_previews: &mut WriteStorage<'_, ApwPreview>,
        asset_selection_parents: &mut WriteStorage<'_, AssetSelectionParent>,
        character_preview_spawn_resources: &mut CharacterPreviewSpawnResources,
        ash_entity: Entity,
        apw_main_entity: Option<Entity>,
        asset_selection: AssetSelection,
    ) {
        let CharacterPreviewSpawnResources {
            parent_entities,
            groundings,
            spawn_game_object_resources,
        } = character_preview_spawn_resources;

        if let AssetSelection::Id(asset_id) = asset_selection {
            // TODO: Take in position to spawn entity.
            let x = 60.;
            // Hack: Since characters have `PositionZAsY`, we shift the entity's Y position up by
            // the Z position of the asset_selection_entity.
            let y = 30. + 12.;
            let z = 1.;
            let position = Position::new(x, y, z);

            // TODO: Look up sequence ID for default sequence ID for the asset type.
            let sequence_id = SequenceId::new(0);

            let spawn = Spawn {
                object: asset_id,
                position,
                velocity: Velocity::default(),
                sequence_id,
            };

            let spawn_parent_entity = apw_main_entity.unwrap_or(ash_entity);
            let entity_spawned =
                GameObjectSpawner::spawn(spawn_game_object_resources, spawn_parent_entity, &spawn);

            apw_previews
                .insert(entity_spawned, ApwPreview)
                .expect("Failed to insert `ApwPreview` component.");
            asset_selection_parents
                .insert(entity_spawned, AssetSelectionParent::new(ash_entity))
                .expect("Failed to insert `AssetSelectionParent` component.");
            parent_entities
                .insert(entity_spawned, ParentEntity::new(ash_entity))
                .expect("Failed to insert `ParentEntity` component.");
            groundings
                .insert(entity_spawned, Grounding::OnGround)
                .expect("Failed to insert `Grounding` component.");
        }
    }
}
