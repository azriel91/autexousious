use amethyst::{
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_selection_model::play::{AssetSelection, AssetSelectionEvent};
use asset_ui_model::{config::AswPortraitName, loaded::AswPortraits, play::AssetSelectionParent};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    config::ControllerId,
    play::{InputControlled, SharedInputControlled},
};
use sequence_model::loaded::SequenceId;

/// Switches `AssetSelectionWidget` portrait entity when receving a
/// `AssetSelectionEvent`.
#[derive(Debug, Default, new)]
pub struct AswPortraitUpdateSystem {
    /// Reader ID for the `AssetSelectionEvent` channel.
    #[new(default)]
    asset_selection_event_rid: Option<ReaderId<AssetSelectionEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AswPortraitUpdateSystemData<'s> {
    /// `AssetSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub asset_selection_ec: Read<'s, EventChannel<AssetSelectionEvent>>,
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `SharedInputControlled` components.
    #[derivative(Debug = "ignore")]
    pub shared_input_controlleds: ReadStorage<'s, SharedInputControlled>,
    /// `AssetSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_parents: ReadStorage<'s, AssetSelectionParent>,
    /// `AswPortraits` components.
    #[derivative(Debug = "ignore")]
    pub asw_portraitses: ReadStorage<'s, AswPortraits>,
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: ReadStorage<'s, AssetSelection>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl AswPortraitUpdateSystem {
    /// Finds the portrait `Entity` with the given controller ID.
    ///
    /// Returns the entity and its `AssetSelectionParent` if found.
    fn find_asw_portrait<'s>(
        (
            entities,
            input_controlleds,
            shared_input_controlleds,
            asw_portraitses,
            asset_selection_parents,
        ): (
            &'s Entities<'_>,
            &'s ReadStorage<'_, InputControlled>,
            &'s ReadStorage<'_, SharedInputControlled>,
            &'s ReadStorage<'_, AswPortraits>,
            &'s ReadStorage<'_, AssetSelectionParent>,
        ),
        controller_id: ControllerId,
    ) -> Option<(Entity, &'s AswPortraits, AssetSelectionParent)> {
        (
            entities,
            input_controlleds,
            asw_portraitses,
            asset_selection_parents,
        )
            .join()
            .find_map(
                |(entity_portrait, input_controlled, asw_portraits, asset_selection_parent)| {
                    if input_controlled.controller_id == controller_id {
                        Some((entity_portrait, asw_portraits, *asset_selection_parent))
                    } else {
                        None
                    }
                },
            )
            .or_else(|| {
                (
                    entities,
                    shared_input_controlleds,
                    asw_portraitses,
                    asset_selection_parents,
                )
                    .join()
                    .map(
                        |(entity_portrait, _, asw_portraits, asset_selection_parent)| {
                            (entity_portrait, asw_portraits, *asset_selection_parent)
                        },
                    )
                    .next()
            })
    }
}

impl<'s> System<'s> for AswPortraitUpdateSystem {
    type SystemData = AswPortraitUpdateSystemData<'s>;

    fn run(&mut self, mut csw_portrait_update_system_data: Self::SystemData) {
        let AswPortraitUpdateSystemData {
            ref asset_selection_ec,
            ref entities,
            ref input_controlleds,
            ref shared_input_controlleds,
            ref asset_selection_parents,
            ref asw_portraitses,
            ref asset_selections,
            ref mut sequence_ids,
        } = csw_portrait_update_system_data;

        let find_csw_data = (
            entities,
            input_controlleds,
            shared_input_controlleds,
            asw_portraitses,
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
                    if let Some((entity_portrait, asw_portraits, asset_selection)) =
                        Self::find_asw_portrait(find_csw_data, *controller_id).and_then(
                            |(entity_portrait, asw_portraits, asset_selection_parent)| {
                                asset_selections.get(asset_selection_parent.0).map(
                                    |asset_selection| {
                                        (entity_portrait, asw_portraits, asset_selection)
                                    },
                                )
                            },
                        )
                    {
                        let sequence_id = match asset_selection {
                            AssetSelection::Random => asw_portraits.get(&AswPortraitName::Random),
                            AssetSelection::Id(_asset_id) => {
                                asw_portraits.get(&AswPortraitName::Select)
                            }
                        }
                        .copied();
                        if let Some(sequence_id) = sequence_id {
                            sequence_ids
                                .insert(entity_portrait, sequence_id)
                                .expect("Failed to insert `SequenceId` component.");
                        }
                    }
                }
                AssetSelectionEvent::Leave { controller_id, .. } => {
                    if let Some((entity_portrait, asw_portraits, _)) =
                        Self::find_asw_portrait(find_csw_data, *controller_id)
                    {
                        let sequence_id = asw_portraits.get(&AswPortraitName::Join).copied();
                        if let Some(sequence_id) = sequence_id {
                            sequence_ids
                                .insert(entity_portrait, sequence_id)
                                .expect("Failed to insert `SequenceId` component.");
                        }
                    }
                }
                AssetSelectionEvent::Switch {
                    controller_id,
                    asset_selection,
                    ..
                } => {
                    if let Some((entity_portrait, asw_portraits, _)) =
                        Self::find_asw_portrait(find_csw_data, *controller_id)
                    {
                        let sequence_id = match asset_selection {
                            AssetSelection::Random => asw_portraits.get(&AswPortraitName::Random),
                            AssetSelection::Id(_asset_id) => {
                                asw_portraits.get(&AswPortraitName::Select)
                            }
                        }
                        .copied();
                        if let Some(sequence_id) = sequence_id {
                            sequence_ids
                                .insert(entity_portrait, sequence_id)
                                .expect("Failed to insert `SequenceId` component.");
                        }
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
