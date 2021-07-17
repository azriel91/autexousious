mod character_preview_spawn;
mod map_preview_spawn;

use std::{fmt::Debug, marker::PhantomData};

use amethyst::{
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{config::AssetType, loaded::AssetTypeMappings};
use asset_selection_model::play::{AssetSelection, AssetSelectionEvent};
use asset_selection_ui_model::play::{ApwMain, ApwPreview};
use asset_ui_model::play::{AssetSelectionHighlightMain, AssetSelectionParent};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    config::ControllerId,
    play::{InputControlled, SharedInputControlled},
};
use log::error;

use self::{character_preview_spawn::CharacterPreviewSpawn, map_preview_spawn::MapPreviewSpawn};

/// Trait of different asset preview widget spawn behaviours.
pub trait PreviewSpawner<'s> {
    type SystemData: SystemData<'s>;
    const ASSET_TYPE: AssetType;

    fn spawn_preview_entities(
        apw_previews: &mut WriteStorage<'_, ApwPreview>,
        asset_selection_parents: &mut WriteStorage<'_, AssetSelectionParent>,
        preview_spawn_resources: &mut Self::SystemData,
        ash_entity: Entity,
        apw_main_entity: Option<Entity>,
        asset_selection: AssetSelection,
    );
}

/// System to spawn character previews.
pub type ApwPreviewSpawnSystemCharacter = ApwPreviewSpawnSystem<CharacterPreviewSpawn>;

/// System to spawn map previews.
pub type ApwPreviewSpawnSystemMap = ApwPreviewSpawnSystem<MapPreviewSpawn>;

/// Spawns / deletes character preview entities when character selection is
/// switched.
#[derive(Debug, Default, new)]
pub struct ApwPreviewSpawnSystem<PS> {
    /// Reader ID for the `AssetSelectionEvent` channel.
    #[new(default)]
    asset_selection_event_rid: Option<ReaderId<AssetSelectionEvent>>,
    /// Marker.
    marker: PhantomData<PS>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ApwPreviewSpawnSystemData<'s, PS>
where
    PS: for<'ps> PreviewSpawner<'ps>,
{
    /// `AssetSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub asset_selection_ec: Read<'s, EventChannel<AssetSelectionEvent>>,
    /// `ApwPreviewSpawnResources`.
    pub apw_preview_spawn_resources: ApwPreviewSpawnResources<'s, PS>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ApwPreviewSpawnResources<'s, PS>
where
    PS: for<'ps> PreviewSpawner<'ps>,
{
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
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
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: ReadStorage<'s, AssetSelection>,
    /// `PreviewSpawnResources`.
    #[derivative(Debug = "ignore")]
    pub preview_spawn_resources: <PS as PreviewSpawner<'s>>::SystemData,
}

impl<PS> ApwPreviewSpawnSystem<PS>
where
    PS: for<'ps> PreviewSpawner<'ps>,
{
    /// Finds the main asset preview widget `Entity` with the given controller
    /// ID.
    fn find_apw_main_entity(
        ApwPreviewSpawnResources {
            entities,
            apw_mains,
            input_controlleds,
            shared_input_controlleds,
            ..
        }: &ApwPreviewSpawnResources<PS>,
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

    /// Finds the main asset selection cell `Entity` that the ASH entity is
    /// attached to.
    fn find_asset_selection_highlight_entity(
        ApwPreviewSpawnResources {
            entities,
            ash_mains,
            input_controlleds,
            ..
        }: &ApwPreviewSpawnResources<PS>,
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
        apw_preview_spawn_resources: &ApwPreviewSpawnResources<PS>,
        ash_entity: Entity,
    ) {
        let ApwPreviewSpawnResources {
            entities,
            asset_type_mappings,
            apw_previews,
            asset_selection_parents,
            asset_selections,
            ..
        } = apw_preview_spawn_resources;

        // Need to not delete entities that are just spawned.
        (entities, apw_previews, asset_selection_parents)
            .join()
            .filter_map(|(entity, _, asset_selection_parent)| {
                if asset_selection_parent.0 == ash_entity {
                    Some((entity, asset_selection_parent.0))
                } else {
                    None
                }
            })
            .filter_map(|(entity, asset_selection_parent_entity)| {
                let asset_type = asset_selections
                    .get(asset_selection_parent_entity)
                    .copied()
                    .and_then(|asset_selection| {
                        if let AssetSelection::Id(asset_id) = asset_selection {
                            Some(asset_id)
                        } else {
                            None
                        }
                    })
                    .and_then(|asset_id| asset_type_mappings.get(asset_id))
                    .copied();
                if let Some(asset_type) = asset_type {
                    if asset_type == PS::ASSET_TYPE {
                        Some(entity)
                    } else {
                        None
                    }
                } else {
                    Some(entity)
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
        apw_preview_spawn_resources: &mut ApwPreviewSpawnResources<PS>,
        ash_entity: Entity,
        controller_id: ControllerId,
        asset_selection: Option<AssetSelection>,
    ) {
        let apw_main_entity =
            Self::find_apw_main_entity(&apw_preview_spawn_resources, controller_id);

        let ApwPreviewSpawnResources {
            asset_type_mappings,
            apw_previews,
            asset_selection_parents,
            asset_selections,
            preview_spawn_resources,
            ..
        } = apw_preview_spawn_resources;

        let asset_selection = asset_selection.or_else(|| asset_selections.get(ash_entity).copied());

        if let Some(asset_selection) = asset_selection {
            if let AssetSelection::Id(asset_id) = asset_selection {
                let asset_type = asset_type_mappings.get(asset_id).copied();
                if let Some(asset_type) = asset_type {
                    if asset_type == PS::ASSET_TYPE {
                        PS::spawn_preview_entities(
                            apw_previews,
                            asset_selection_parents,
                            preview_spawn_resources,
                            ash_entity,
                            apw_main_entity,
                            asset_selection,
                        );
                    }
                }
            }
        }
    }
}

impl<'s, PS> System<'s> for ApwPreviewSpawnSystem<PS>
where
    PS: for<'ps> PreviewSpawner<'ps>,
{
    type SystemData = ApwPreviewSpawnSystemData<'s, PS>;

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
