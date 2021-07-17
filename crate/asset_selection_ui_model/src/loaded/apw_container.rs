use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entities, Entity, Read, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::ItemId, ItemComponent};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    loaded::PlayerControllers,
    play::{InputControlled, SharedInputControlled},
};
use parent_model::play::ParentEntity;

/// Creates a varying number of `AssetPreviewWidget`s depending on number of
/// `PlayerControllers`.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub enum ApwContainer {
    /// Each `AssetPreviewWidget` is controlled by its own `InputControlled`.
    Individual {
        /// `ItemId`s of each `AssetPreviewWidget`.
        apw_item_ids: Vec<ItemId>,
    },
    /// One `AssetPreviewWidget` is controlled by all controllers.
    Shared {
        /// `ItemId` of the `AssetPreviewWidget` to be `SharedInputControlled`..
        apw_item_id: ItemId,
    },
}

/// `AssetPreviewWidgetSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetPreviewWidgetSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `PlayerControllers` resource.
    #[derivative(Debug = "ignore")]
    pub player_controllers: Read<'s, PlayerControllers>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
    /// `SharedInputControlled` components.
    #[derivative(Debug = "ignore")]
    pub shared_input_controlleds: WriteStorage<'s, SharedInputControlled>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
}

impl<'s> ItemComponent<'s> for ApwContainer {
    type SystemData = AssetPreviewWidgetSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AssetPreviewWidgetSystemData {
            entities,
            player_controllers,
            item_ids,
            input_controlleds,
            shared_input_controlleds,
            parent_entities,
        } = system_data;

        let parent_entity = ParentEntity::new(entity);

        match self {
            Self::Individual { apw_item_ids } => {
                let input_controlled_components = player_controllers
                    .iter()
                    .map(|player_controller| InputControlled::new(player_controller.controller_id));

                apw_item_ids
                    .iter()
                    .copied()
                    .zip(input_controlled_components)
                    .for_each(|(apw_item_id, input_controlled)| {
                        entities
                            .build_entity()
                            .with(parent_entity, parent_entities)
                            .with(apw_item_id, item_ids)
                            .with(input_controlled, input_controlleds)
                            .build();
                    });
            }
            Self::Shared { apw_item_id } => {
                entities
                    .build_entity()
                    .with(parent_entity, parent_entities)
                    .with(*apw_item_id, item_ids)
                    .with(SharedInputControlled, shared_input_controlleds)
                    .build();

                // Even when the `AssetSelectionHighlight` entities use a
                // `SharedInputControlled`, we still need entities with
                // `InputControlled` to receive `ControllerInput`s.
                player_controllers
                    .iter()
                    .map(|player_controller| InputControlled::new(player_controller.controller_id))
                    .for_each(|input_controlled| {
                        entities
                            .build_entity()
                            .with(parent_entity, parent_entities)
                            .with(input_controlled, input_controlleds)
                            .build();
                    });
            }
        }
    }
}
