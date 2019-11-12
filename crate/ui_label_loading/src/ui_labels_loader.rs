use asset_model::loaded::AssetId;
use derivative::Derivative;

use ui_label_model::{
    config::UiLabel,
    loaded::{AssetUiLabels, UiLabels},
};

/// Loads `UiLabel`s from items.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct UiLabelsLoader<'s> {
    /// `AssetUiLabels`.
    pub asset_ui_labels: &'s mut AssetUiLabels,
}

impl<'s> UiLabelsLoader<'s> {
    /// Loads `UiLabels`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the asset data.
    /// * `asset_id`: Asset ID to store the asset data against.
    pub fn load<ItemIterator, ItemRef>(&mut self, item_iterator: ItemIterator, asset_id: AssetId)
    where
        ItemIterator: Iterator<Item = ItemRef>,
        ItemRef: AsRef<UiLabel>,
    {
        let ui_labels = Self::items_to_datas(item_iterator);

        self.asset_ui_labels.insert(asset_id, ui_labels);
    }

    /// Maps items to `UiLabels`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the asset data.
    pub fn items_to_datas<ItemIterator, ItemRef>(item_iterator: ItemIterator) -> UiLabels
    where
        ItemIterator: Iterator<Item = ItemRef>,
        ItemRef: AsRef<UiLabel>,
    {
        let ui_labels = item_iterator
            .map(Self::item_to_data)
            .collect::<Vec<UiLabel>>();

        UiLabels::new(ui_labels)
    }

    /// Maps the item to the data.
    ///
    /// # Parameters
    ///
    /// * `item_ref`: Reference to the item.
    pub fn item_to_data<ItemRef>(item_ref: ItemRef) -> UiLabel
    where
        ItemRef: AsRef<UiLabel>,
    {
        AsRef::<UiLabel>::as_ref(&item_ref).clone()
    }
}
