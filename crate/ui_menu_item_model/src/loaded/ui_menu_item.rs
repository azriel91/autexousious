use std::fmt::Debug;

use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use menu_model::MenuItem;

/// Defines a UI menu item.
#[derive(Clone, Debug, Component, PartialEq, new)]
pub struct UiMenuItem<I>
where
    I: Copy + Debug + PartialEq + Send + Sync + 'static,
{
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
}

impl<'s, I> ItemComponent<'s> for UiMenuItem<I>
where
    I: Copy + Debug + PartialEq + Send + Sync + 'static,
{
    type SystemData = UiMenuItemSystemData<'s, I>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let UiMenuItemSystemData { menu_items } = system_data;

        menu_items
            .insert(entity, MenuItem::new(self.index))
            .expect("Failed to insert `SequenceId` component.");
    }
}
