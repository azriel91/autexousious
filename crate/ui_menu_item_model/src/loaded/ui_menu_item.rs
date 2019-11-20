use std::fmt::Debug;

use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use application_menu::{MenuItem, MenuItemWidgetState};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use sequence_model::loaded::SequenceId;
use typename_derive::TypeName;

/// Defines a UI menu item.
#[derive(Clone, Debug, Component, PartialEq, TypeName, new)]
pub struct UiMenuItem<I>
where
    I: Copy + Debug + PartialEq + Send + Sync + 'static,
{
    /// `SequenceId` that the `UIMenuItem` should begin with.
    pub sequence_id: SequenceId,
    /// Menu index this item corresponds to.
    pub index: I,
}

/// `UiMenuItemSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiMenuItemSystemData<'s, I>
where
    I: Copy + Debug + PartialEq + Send + Sync + 'static,
{
    /// `MenuItem<GameModeIndex>` components.
    #[derivative(Debug = "ignore")]
    pub menu_items: WriteStorage<'s, MenuItem<I>>,
    /// `MenuItemWidgetState` components.
    #[derivative(Debug = "ignore")]
    pub menu_item_widget_states: WriteStorage<'s, MenuItemWidgetState>,
}

impl<'s, I> ItemComponent<'s> for UiMenuItem<I>
where
    I: Copy + Debug + PartialEq + Send + Sync + 'static,
{
    type SystemData = UiMenuItemSystemData<'s, I>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let UiMenuItemSystemData {
            menu_items,
            menu_item_widget_states,
        } = system_data;

        menu_items
            .insert(entity, MenuItem::new(self.index))
            .expect("Failed to insert `SequenceId` component.");
        menu_item_widget_states
            .insert(entity, MenuItemWidgetState::Idle)
            .expect("Failed to insert `MenuItemWidgetState` component.");
    }
}
