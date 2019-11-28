use game_mode_selection_model::GameModeIndex;
use ui_menu_item_model::{
    config,
    loaded::{UiMenuItem, UiMenuItems},
};

/// Loads `UiMenuItem`s from items.
#[derive(Debug)]
pub struct UiMenuItemsLoader;

impl UiMenuItemsLoader {
    /// Loads `UiMenuItems`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the asset data.
    pub fn items_to_datas<'f, ItemIterator>(
        item_iterator: ItemIterator,
    ) -> UiMenuItems<GameModeIndex>
    where
        ItemIterator: Iterator<Item = &'f config::UiMenuItem<GameModeIndex>>,
    {
        let ui_menu_items = item_iterator
            .map(|ui_menu_item| UiMenuItem::new(ui_menu_item.index))
            .collect::<Vec<UiMenuItem<GameModeIndex>>>();

        UiMenuItems::new(ui_menu_items)
    }
}
