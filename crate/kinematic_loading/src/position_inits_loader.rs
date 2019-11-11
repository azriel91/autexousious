use asset_model::loaded::AssetId;
use derivative::Derivative;
use kinematic_model::{
    config::PositionInit,
    loaded::{AssetPositionInits, PositionInits},
};

/// Loads `PositionInit`s from items.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct PositionInitsLoader<'s> {
    /// `AssetPositionInits`.
    pub asset_position_inits: &'s mut AssetPositionInits,
}

impl<'s> PositionInitsLoader<'s> {
    /// Loads `PositionInits`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the asset data.
    /// * `asset_id`: Asset ID to store the asset data against.
    pub fn load<ItemIterator, ItemRef>(&mut self, item_iterator: ItemIterator, asset_id: AssetId)
    where
        ItemIterator: Iterator<Item = ItemRef>,
        ItemRef: AsRef<PositionInit>,
    {
        let position_inits = Self::items_to_datas(item_iterator);

        self.asset_position_inits.insert(asset_id, position_inits);
    }

    /// Maps items to `PositionInits`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the asset data.
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
