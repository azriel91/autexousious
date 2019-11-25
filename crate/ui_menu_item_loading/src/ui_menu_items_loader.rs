use asset_model::loaded::AssetIdMappings;
use game_mode_selection_model::GameModeIndex;
use sequence_model::loaded::AssetSequenceIdMappings;
use sprite_model::config::SpriteSequenceName;
use ui_menu_item_model::{
    config,
    loaded::{UiMenuItem, UiMenuItems},
};

/// Loads `UiMenuItem`s from items.
#[derive(Debug)]
pub struct UiMenuItemsLoader<'s> {
    /// `AssetIdMappings`.
    pub asset_id_mappings: &'s AssetIdMappings,
    /// `AssetSequenceIdMappings`.
    pub asset_sequence_id_mappings_sprite: &'s AssetSequenceIdMappings<SpriteSequenceName>,
}

impl<'s> UiMenuItemsLoader<'s> {
    /// Loads `UiMenuItems`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the asset data.
    pub fn items_to_datas<'f, ItemIterator>(
        &self,
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
