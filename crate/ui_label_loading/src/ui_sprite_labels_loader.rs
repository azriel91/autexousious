use asset_model::loaded::{AssetId, AssetIdMappings};
use sequence_model::loaded::AssetSequenceIdMappings;
use sprite_model::config::SpriteSequenceName;
use ui_label_model::{
    config,
    loaded::{UiSpriteLabel, UiSpriteLabels},
};

/// Loads `UiSpriteLabel`s from items.
#[derive(Debug)]
pub struct UiSpriteLabelsLoader<'s> {
    /// `AssetIdMappings`.
    pub asset_id_mappings: &'s AssetIdMappings,
    /// `AssetSequenceIdMappings`.
    pub asset_sequence_id_mappings_sprite: &'s AssetSequenceIdMappings<SpriteSequenceName>,
}

impl<'s> UiSpriteLabelsLoader<'s> {
    /// Loads `UiSpriteLabels`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the asset data.
    /// * `asset_id`: Asset ID to store the asset data against.
    pub fn items_to_datas<ItemIterator, ItemRef>(
        &self,
        item_iterator: ItemIterator,
        asset_id: AssetId,
    ) -> UiSpriteLabels
    where
        ItemIterator: Iterator<Item = ItemRef>,
        ItemRef: AsRef<config::UiSpriteLabel>,
    {
        let ui_sprite_labels = item_iterator
            .map(|item_ref| {
                Self::item_to_data(
                    self.asset_id_mappings,
                    self.asset_sequence_id_mappings_sprite,
                    asset_id,
                    item_ref,
                )
            })
            .collect::<Vec<UiSpriteLabel>>();

        UiSpriteLabels::new(ui_sprite_labels)
    }

    /// Maps the item to the data.
    ///
    /// # Parameters
    ///
    /// * `item_ref`: Reference to the item.
    pub fn item_to_data<ItemRef>(
        asset_id_mappings: &AssetIdMappings,
        asset_sequence_id_mappings_sprite: &AssetSequenceIdMappings<SpriteSequenceName>,
        asset_id: AssetId,
        item_ref: ItemRef,
    ) -> UiSpriteLabel
    where
        ItemRef: AsRef<config::UiSpriteLabel>,
    {
        let ui_sprite_label = AsRef::<config::UiSpriteLabel>::as_ref(&item_ref);

        let sequence_id_mappings = asset_sequence_id_mappings_sprite
            .get(asset_id)
            .unwrap_or_else(|| {
                let asset_slug = asset_id_mappings
                    .slug(asset_id)
                    .expect("Expected `AssetSlug` to exist.");
                panic!(
                    "Expected `SequenceIdMappings<SpriteSequenceName>` to exist for `{}`.",
                    asset_slug
                )
            });
        let sequence = &ui_sprite_label.sequence;
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
        UiSpriteLabel::new(ui_sprite_label.position, sequence_id)
    }
}
