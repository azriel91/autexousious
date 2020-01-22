use amethyst::{
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::play::{AssetSelection, AssetSelectionEvent};
use asset_selection_ui_model::play::{ApwMain, ApwPreview};
use asset_ui_model::play::{AssetSelectionHighlightMain, AssetSelectionParent};
use derivative::Derivative;
use derive_new::new;
use game_input::{InputControlled, SharedInputControlled};
use game_input_model::ControllerId;
use kinematic_model::config::{Position, Velocity};
use log::error;
use object_model::play::Grounding;
use parent_model::play::ParentEntity;
use sequence_model::loaded::SequenceId;
use spawn_model::loaded::Spawn;
use spawn_play::{GameObjectSpawner, SpawnGameObjectResources};

/// Spawns / deletes character preview entities when character selection is switched.
#[derive(Debug, Default, new)]
pub struct ApwPreviewSpawnSystem {
    /// Reader ID for the `AssetSelectionEvent` channel.
    #[new(default)]
    asset_selection_event_rid: Option<ReaderId<AssetSelectionEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ApwPreviewSpawnSystemData<'s> {
    /// `AssetSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub asset_selection_ec: Read<'s, EventChannel<AssetSelectionEvent>>,
    /// `ApwPreviewSpawnResources`.
    pub apw_preview_spawn_resources: ApwPreviewSpawnResources<'s>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ApwPreviewSpawnResources<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ApwMain` components.
    #[derivative(Debug = "ignore")]
    pub apw_mains: ReadStorage<'s, ApwMain>,
    /// `AssetSelectionHighlightMain` components.
    #[derivative(Debug = "ignore")]
    pub ash_mains: ReadStorage<'s, AssetSelectionHighlightMain>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `SharedInputControlled` components.
    #[derivative(Debug = "ignore")]
    pub shared_input_controlleds: ReadStorage<'s, SharedInputControlled>,
    /// `ApwPreview` components.
    #[derivative(Debug = "ignore")]
    pub apw_previews: WriteStorage<'s, ApwPreview>,
    /// `AssetSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_parents: WriteStorage<'s, AssetSelectionParent>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: ReadStorage<'s, AssetSelection>,
    /// `SpawnGameObjectResources`.
    pub spawn_game_object_resources: SpawnGameObjectResources<'s>,
}

impl ApwPreviewSpawnSystem {
    /// Finds the main asset preview widget `Entity` with the given controller ID.
    fn find_apw_main_entity(
        ApwPreviewSpawnResources {
            entities,
            apw_mains,
            input_controlleds,
            shared_input_controlleds,
            ..
        }: &ApwPreviewSpawnResources,
        controller_id: ControllerId,
    ) -> Option<Entity> {
        (entities, apw_mains, input_controlleds)
            .join()
            .find_map(|(entity, _, input_controlled)| {
                if input_controlled.controller_id == controller_id {
                    Some(entity)
                } else {
                    None
                }
            })
            .or_else(|| {
                (entities, apw_mains, shared_input_controlleds)
                    .join()
                    .map(|(entity, _, _)| entity)
                    .next()
            })
    }

    /// Finds the main asset selection cell `Entity` that the ASH entity is attached to.
    fn find_asset_selection_highlight_entity(
        ApwPreviewSpawnResources {
            entities,
            ash_mains,
            input_controlleds,
            ..
        }: &ApwPreviewSpawnResources,
        controller_id: ControllerId,
    ) -> Option<Entity> {
        (entities, ash_mains, input_controlleds)
            .join()
            .find_map(|(entity, _, input_controlled)| {
                if input_controlled.controller_id == controller_id {
                    Some(entity)
                } else {
                    None
                }
            })
    }

    /// Deletes `ApwPreview` entities for a particular ASH entity.
    fn delete_preview_entities(
        apw_preview_spawn_resources: &ApwPreviewSpawnResources,
        ash_entity: Entity,
    ) {
        let ApwPreviewSpawnResources {
            entities,
            apw_previews,
            asset_selection_parents,
            ..
        } = apw_preview_spawn_resources;

        (entities, apw_previews, asset_selection_parents)
            .join()
            .filter_map(|(entity, _, asset_selection_parent)| {
                if asset_selection_parent.0 == ash_entity {
                    Some(entity)
                } else {
                    None
                }
            })
            .for_each(|entity| {
                if let Err(e) = entities.delete(entity) {
                    error!("Failed to delete entity: {}", e);
                }
            });
    }

    // Spawns new entities that provide a preview for the asset preview widget.
    fn spawn_preview_entities(
        apw_preview_spawn_resources: &mut ApwPreviewSpawnResources,
        ash_entity: Entity,
        controller_id: ControllerId,
        asset_selection: Option<AssetSelection>,
    ) {
        let apw_main_entity =
            Self::find_apw_main_entity(&apw_preview_spawn_resources, controller_id);

        let ApwPreviewSpawnResources {
            apw_previews,
            asset_selection_parents,
            parent_entities,
            groundings,
            asset_selections,
            spawn_game_object_resources,
            ..
        } = apw_preview_spawn_resources;

        let asset_selection = asset_selection.or_else(|| asset_selections.get(ash_entity).copied());

        if let Some(AssetSelection::Id(asset_id)) = asset_selection {
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

impl<'s> System<'s> for ApwPreviewSpawnSystem {
    type SystemData = ApwPreviewSpawnSystemData<'s>;

    fn run(&mut self, mut apw_preview_spawn_system_data: Self::SystemData) {
        let ApwPreviewSpawnSystemData {
            ref asset_selection_ec,
            ref mut apw_preview_spawn_resources,
        } = apw_preview_spawn_system_data;

        let asset_selection_event_rid = self
            .asset_selection_event_rid
            .as_mut()
            .expect("Expected `asset_selection_event_rid` field to be set.");

        asset_selection_ec
            .read(asset_selection_event_rid)
            .for_each(|ev| match *ev {
                AssetSelectionEvent::Return => {}
                AssetSelectionEvent::Join {
                    entity,
                    controller_id,
                } => {
                    let ash_entity = entity.or_else(|| {
                        Self::find_asset_selection_highlight_entity(
                            apw_preview_spawn_resources,
                            controller_id,
                        )
                    });
                    if let Some(ash_entity) = ash_entity {
                        Self::spawn_preview_entities(
                            apw_preview_spawn_resources,
                            ash_entity,
                            controller_id,
                            None,
                        );
                    }
                }
                AssetSelectionEvent::Leave {
                    entity,
                    controller_id,
                } => {
                    let ash_entity = entity.or_else(|| {
                        Self::find_asset_selection_highlight_entity(
                            apw_preview_spawn_resources,
                            controller_id,
                        )
                    });
                    if let Some(ash_entity) = ash_entity {
                        Self::delete_preview_entities(&apw_preview_spawn_resources, ash_entity);
                    }
                }
                AssetSelectionEvent::Switch {
                    entity,
                    controller_id,
                    asset_selection,
                } => {
                    let ash_entity = entity.or_else(|| {
                        Self::find_asset_selection_highlight_entity(
                            apw_preview_spawn_resources,
                            controller_id,
                        )
                    });
                    if let Some(ash_entity) = ash_entity {
                        Self::delete_preview_entities(apw_preview_spawn_resources, ash_entity);
                        Self::spawn_preview_entities(
                            apw_preview_spawn_resources,
                            ash_entity,
                            controller_id,
                            Some(asset_selection),
                        );
                    }
                }
                // No update needed -- preview entities are already correct.
                AssetSelectionEvent::Select { .. } | AssetSelectionEvent::Deselect { .. } => {}
                AssetSelectionEvent::Confirm => {}
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.asset_selection_event_rid = Some(
            world
                .fetch_mut::<EventChannel<AssetSelectionEvent>>()
                .register_reader(),
        );
    }
}
