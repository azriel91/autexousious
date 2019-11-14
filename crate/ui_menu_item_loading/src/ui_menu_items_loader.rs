use asset_model::loaded::{AssetId, AssetIdMappings};
use derivative::Derivative;
use game_mode_selection_model::GameModeIndex;
use sequence_model::loaded::AssetSequenceIdMappings;
use ui_menu_item_model::{
    config,
    loaded::{AssetUiMenuItems, UiMenuItem, UiMenuItems},
};
use ui_model_spi::config::UiSequenceName;

/// Loads `UiMenuItem`s from items.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct UiMenuItemsLoader<'s> {
    /// `AssetIdMappings`.
    pub asset_id_mappings: &'s AssetIdMappings,
    /// `AssetSequenceIdMappings`.
    pub asset_sequence_id_mappings_ui: &'s AssetSequenceIdMappings<UiSequenceName>,
    /// `AssetUiMenuItems`.
    pub asset_ui_menu_items: &'s mut AssetUiMenuItems<GameModeIndex>,
}

impl<'s> UiMenuItemsLoader<'s> {
    /// Loads `UiMenuItems`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the asset data.
    /// * `asset_id`: Asset ID to store the asset data against.
    pub fn load<'f, ItemIterator>(&mut self, item_iterator: ItemIterator, asset_id: AssetId)
    where
        ItemIterator: Iterator<Item = &'f config::UiMenuItem<GameModeIndex>>,
    {
        let ui_menu_items = Self::items_to_datas(
            &self.asset_id_mappings,
            &self.asset_sequence_id_mappings_ui,
            asset_id,
            item_iterator,
        );

        self.asset_ui_menu_items.insert(asset_id, ui_menu_items);
    }

    /// Maps items to `UiMenuItems`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the asset data.
    pub fn items_to_datas<'f, ItemIterator>(
        asset_id_mappings: &AssetIdMappings,
        asset_sequence_id_mappings_ui: &AssetSequenceIdMappings<UiSequenceName>,
        asset_id: AssetId,
        item_iterator: ItemIterator,
    ) -> UiMenuItems<GameModeIndex>
    where
        ItemIterator: Iterator<Item = &'f config::UiMenuItem<GameModeIndex>>,
    {
        let ui_menu_items = item_iterator
            .map(|ui_menu_item| {
                let sequence_id_mappings = asset_sequence_id_mappings_ui
                    .get(asset_id)
                    .unwrap_or_else(|| {
                        let asset_slug = asset_id_mappings
                            .slug(asset_id)
                            .expect("Expected `AssetSlug` to exist.");
                        panic!(
                            "Expected `SequenceIdMappings<UiSequenceName>` to exist for `{}`.",
                            asset_slug
                        )
                    });
                let sequence = &ui_menu_item.sprite.sequence;
                let sequence_id = sequence_id_mappings
                    .id(sequence)
                    .copied()
                    .unwrap_or_else(|| {
                        let asset_slug = asset_id_mappings
                            .slug(asset_id)
                            .expect("Expected `AssetSlug` to exist.");
                        panic!(
                            "Expected `SequenceIdMapping` to exist for sequence `{}` for asset \
                             `{}`.",
                            sequence, asset_slug
                        )
                    });

                UiMenuItem::new(sequence_id, ui_menu_item.index)
            })
            .collect::<Vec<UiMenuItem<GameModeIndex>>>();

        UiMenuItems::new(ui_menu_items)
    }
}
