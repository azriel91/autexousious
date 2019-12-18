use amethyst::{
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
use character_selection_ui_model::play::{CharacterSelectionParent, CswMain, CswPreview};
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
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
pub struct CswPreviewSpawnSystem {
    /// Reader ID for the `CharacterSelectionEvent` channel.
    #[new(default)]
    character_selection_event_rid: Option<ReaderId<CharacterSelectionEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CswPreviewSpawnSystemData<'s> {
    /// `CharacterSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub character_selection_ec: Read<'s, EventChannel<CharacterSelectionEvent>>,
    /// `CswPreviewSpawnResources`.
    pub csw_preview_spawn_resources: CswPreviewSpawnResources<'s>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CswPreviewSpawnResources<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `CswMain` components.
    #[derivative(Debug = "ignore")]
    pub csw_mains: ReadStorage<'s, CswMain>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `CswPreview` components.
    #[derivative(Debug = "ignore")]
    pub csw_previews: WriteStorage<'s, CswPreview>,
    /// `CharacterSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub character_selection_parents: WriteStorage<'s, CharacterSelectionParent>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
    /// `CharacterSelection` components.
    #[derivative(Debug = "ignore")]
    pub character_selections: ReadStorage<'s, CharacterSelection>,
    /// `SpawnGameObjectResources`.
    pub spawn_game_object_resources: SpawnGameObjectResources<'s>,
}

impl CswPreviewSpawnSystem {
    /// Finds the main character selectoin widget `Entity` with the given controller ID.
    fn find_csw_main_entity(
        CswPreviewSpawnResources {
            entities,
            csw_mains,
            input_controlleds,
            ..
        }: &CswPreviewSpawnResources,
        controller_id: ControllerId,
    ) -> Option<Entity> {
        (entities, csw_mains, input_controlleds)
            .join()
            .find_map(|(entity, _, input_controlled)| {
                if input_controlled.controller_id == controller_id {
                    Some(entity)
                } else {
                    None
                }
            })
    }

    // Deletes `CswPreview` entities belonging to the character selection widget.
    fn delete_preview_entities(
        csw_preview_spawn_resources: &CswPreviewSpawnResources,
        controller_id: ControllerId,
        csw_main_entity: Option<Entity>,
    ) {
        let CswPreviewSpawnResources {
            entities,
            csw_previews,
            character_selection_parents,
            ..
        } = csw_preview_spawn_resources;

        let csw_main_entity = csw_main_entity
            .or_else(|| Self::find_csw_main_entity(csw_preview_spawn_resources, controller_id));

        // TODO: how do we link the spawned entity with the `CswMain` entity?
        //
        // We can attach `CswPreview` when we receive a `SpawnEvent` in another system, but we don't
        // know which `CswMain` entity it should be attached to.
        //
        // Should we split the logic to spawn game objects into `GameObjectSpawner`, and attach the
        // components that way?
        if let Some(csw_main_entity) = csw_main_entity {
            (entities, csw_previews, character_selection_parents)
                .join()
                .filter_map(|(entity, _, character_selection_parent)| {
                    if character_selection_parent.0 == csw_main_entity {
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
    }

    // Spawns new entities that provide a preview for the character selection widget.
    fn spawn_preview_entities(
        csw_preview_spawn_resources: &mut CswPreviewSpawnResources,
        controller_id: ControllerId,
        csw_main_entity: Option<Entity>,
        character_selection: Option<CharacterSelection>,
    ) {
        let csw_main_entity = csw_main_entity
            .or_else(|| Self::find_csw_main_entity(&csw_preview_spawn_resources, controller_id));

        let CswPreviewSpawnResources {
            csw_previews,
            character_selection_parents,
            parent_entities,
            groundings,
            character_selections,
            spawn_game_object_resources,
            ..
        } = csw_preview_spawn_resources;

        let character_selection = character_selection.or_else(|| {
            csw_main_entity
                .and_then(|csw_main_entity| character_selections.get(csw_main_entity).copied())
        });

        if let (Some(csw_main_entity), Some(CharacterSelection::Id(asset_id))) =
            (csw_main_entity, character_selection)
        {
            // TODO: Take in position to spawn entity.
            let x = 60.;
            // Hack: Since characters have `PositionZAsY`, we shift the entity's Y position up by
            // the Z position of the csw_main_entity.
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
            let entity_spawned =
                GameObjectSpawner::spawn(spawn_game_object_resources, csw_main_entity, &spawn);

            csw_previews
                .insert(entity_spawned, CswPreview)
                .expect("Failed to insert `CswPreview` component.");
            character_selection_parents
                .insert(
                    entity_spawned,
                    CharacterSelectionParent::new(csw_main_entity),
                )
                .expect("Failed to insert `CharacterSelectionParent` component.");
            parent_entities
                .insert(entity_spawned, ParentEntity::new(csw_main_entity))
                .expect("Failed to insert `ParentEntity` component.");
            groundings
                .insert(entity_spawned, Grounding::OnGround)
                .expect("Failed to insert `Grounding` component.");
        }
    }
}

impl<'s> System<'s> for CswPreviewSpawnSystem {
    type SystemData = CswPreviewSpawnSystemData<'s>;

    fn run(&mut self, mut csw_portrait_update_system_data: Self::SystemData) {
        let CswPreviewSpawnSystemData {
            ref character_selection_ec,
            ref mut csw_preview_spawn_resources,
        } = csw_portrait_update_system_data;

        let character_selection_event_rid = self
            .character_selection_event_rid
            .as_mut()
            .expect("Expected `character_selection_event_rid` field to be set.");

        character_selection_ec
            .read(character_selection_event_rid)
            .for_each(|ev| match *ev {
                CharacterSelectionEvent::Return => {}
                CharacterSelectionEvent::Join { controller_id } => {
                    Self::spawn_preview_entities(
                        csw_preview_spawn_resources,
                        controller_id,
                        None,
                        None,
                    );
                }
                CharacterSelectionEvent::Leave { controller_id } => {
                    Self::delete_preview_entities(
                        &csw_preview_spawn_resources,
                        controller_id,
                        None,
                    );
                }
                CharacterSelectionEvent::Switch {
                    controller_id,
                    character_selection,
                } => {
                    let csw_main_entity =
                        Self::find_csw_main_entity(csw_preview_spawn_resources, controller_id);
                    Self::delete_preview_entities(
                        csw_preview_spawn_resources,
                        controller_id,
                        csw_main_entity,
                    );
                    Self::spawn_preview_entities(
                        csw_preview_spawn_resources,
                        controller_id,
                        csw_main_entity,
                        Some(character_selection),
                    );
                }
                // No update needed -- preview entities are already correct.
                CharacterSelectionEvent::Select { .. }
                | CharacterSelectionEvent::Deselect { .. } => {}
                CharacterSelectionEvent::Confirm => {}
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.character_selection_event_rid = Some(
            world
                .fetch_mut::<EventChannel<CharacterSelectionEvent>>()
                .register_reader(),
        );
    }
}
