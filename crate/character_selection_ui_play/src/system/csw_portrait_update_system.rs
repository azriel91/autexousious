use amethyst::{
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::play::{AssetSelection, AssetSelectionEvent};
use asset_ui_model::play::AssetSelectionParent;
use character_selection_ui_model::loaded::CswPortraits;
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_input_model::ControllerId;
use sequence_model::loaded::SequenceId;

/// Switches `AssetSelectionWidget` portrait entity when receving a `AssetSelectionEvent`.
#[derive(Debug, Default, new)]
pub struct CswPortraitUpdateSystem {
    /// Reader ID for the `AssetSelectionEvent` channel.
    #[new(default)]
    asset_selection_event_rid: Option<ReaderId<AssetSelectionEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CswPortraitUpdateSystemData<'s> {
    /// `AssetSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub asset_selection_ec: Read<'s, EventChannel<AssetSelectionEvent>>,
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `AssetSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_parents: ReadStorage<'s, AssetSelectionParent>,
    /// `CswPortraits` components.
    #[derivative(Debug = "ignore")]
    pub csw_portraitses: ReadStorage<'s, CswPortraits>,
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: ReadStorage<'s, AssetSelection>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl CswPortraitUpdateSystem {
    /// Finds the portrait `Entity` with the given controller ID.
    ///
    /// Returns the entity and its `AssetSelectionParent` if found.
    fn find_csw_portrait(
        (entities, input_controlleds, csw_portraitses, asset_selection_parents): (
            &Entities<'_>,
            &ReadStorage<'_, InputControlled>,
            &ReadStorage<'_, CswPortraits>,
            &ReadStorage<'_, AssetSelectionParent>,
        ),
        controller_id: ControllerId,
    ) -> Option<(Entity, CswPortraits, AssetSelectionParent)> {
        (
            entities,
            input_controlleds,
            csw_portraitses,
            asset_selection_parents,
        )
            .join()
            .find_map(
                |(entity_portrait, input_controlled, csw_portraits, asset_selection_parent)| {
                    if input_controlled.controller_id == controller_id {
                        Some((entity_portrait, *csw_portraits, *asset_selection_parent))
                    } else {
                        None
                    }
                },
            )
    }
}

impl<'s> System<'s> for CswPortraitUpdateSystem {
    type SystemData = CswPortraitUpdateSystemData<'s>;

    fn run(&mut self, mut csw_portrait_update_system_data: Self::SystemData) {
        let CswPortraitUpdateSystemData {
            ref asset_selection_ec,
            ref entities,
            ref input_controlleds,
            ref asset_selection_parents,
            ref csw_portraitses,
            ref asset_selections,
            ref mut sequence_ids,
        } = csw_portrait_update_system_data;

        let find_csw_data = (
            entities,
            input_controlleds,
            csw_portraitses,
            asset_selection_parents,
        );

        let asset_selection_event_rid = self
            .asset_selection_event_rid
            .as_mut()
            .expect("Expected `asset_selection_event_rid` field to be set.");

        asset_selection_ec
            .read(asset_selection_event_rid)
            .for_each(|ev| match ev {
                AssetSelectionEvent::Return => {}
                AssetSelectionEvent::Join { controller_id, .. } => {
                    if let Some((entity_portrait, csw_portraits, asset_selection)) =
                        Self::find_csw_portrait(find_csw_data, *controller_id).and_then(
                            |(entity_portrait, csw_portraits, asset_selection_parent)| {
                                asset_selections.get(asset_selection_parent.0).map(
                                    |asset_selection| {
                                        (entity_portrait, csw_portraits, asset_selection)
                                    },
                                )
                            },
                        )
                    {
                        let sequence_id = match asset_selection {
                            AssetSelection::Random => csw_portraits.random,
                            // TODO: Spawn character.
                            AssetSelection::Id(_asset_id) => csw_portraits.select,
                        };
                        sequence_ids
                            .insert(entity_portrait, sequence_id)
                            .expect("Failed to insert `SequenceId` component.");
                    }
                }
                AssetSelectionEvent::Leave { controller_id, .. } => {
                    if let Some((entity_portrait, csw_portraits, _)) =
                        Self::find_csw_portrait(find_csw_data, *controller_id)
                    {
                        let sequence_id = csw_portraits.join;
                        sequence_ids
                            .insert(entity_portrait, sequence_id)
                            .expect("Failed to insert `SequenceId` component.");
                    }
                }
                AssetSelectionEvent::Switch {
                    controller_id,
                    asset_selection,
                    ..
                } => {
                    if let Some((entity_portrait, csw_portraits, _)) =
                        Self::find_csw_portrait(find_csw_data, *controller_id)
                    {
                        let sequence_id = match asset_selection {
                            AssetSelection::Random => csw_portraits.random,
                            // TODO: Spawn character.
                            AssetSelection::Id(_asset_id) => csw_portraits.select,
                        };
                        sequence_ids
                            .insert(entity_portrait, sequence_id)
                            .expect("Failed to insert `SequenceId` component.");
                    }
                }
                // Don't need to update sequence for select / deselect, as they should be on the
                // correct portrait background already.
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
