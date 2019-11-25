use ui_label_model::config::{UiLabel, UiLabels};

/// Loads `UiLabel`s from items.
#[derive(Debug)]
pub struct UiLabelsLoader;

impl UiLabelsLoader {
    /// Loads `UiLabels`.
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
