use kinematic_model::{config::PositionInit, loaded::PositionInits};

/// Loads `PositionInit`s from items.
#[derive(Debug)]
pub struct PositionInitsLoader;

impl PositionInitsLoader {
    /// Loads `PositionInits`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the
    ///   asset data.
    pub fn items_to_datas<ItemIterator, ItemRef>(item_iterator: ItemIterator) -> PositionInits
    where
        ItemIterator: Iterator<Item = ItemRef>,
        ItemRef: AsRef<PositionInit>,
    {
        let position_inits = item_iterator
            .map(Self::item_to_data)
            .collect::<Vec<PositionInit>>();

        PositionInits::new(position_inits)
    }

    /// Maps the item to the data.
    ///
    /// # Parameters
    ///
    /// * `item_ref`: Reference to the item.
    pub fn item_to_data<ItemRef>(item_ref: ItemRef) -> PositionInit
    where
        ItemRef: AsRef<PositionInit>,
    {
        *AsRef::<PositionInit>::as_ref(&item_ref)
    }
}
