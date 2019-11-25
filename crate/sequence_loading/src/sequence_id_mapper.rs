use std::str::FromStr;

use asset_model::{config::AssetSlug, loaded::AssetId};
use log::error;
use sequence_model::{
    config::{SequenceName, SequenceNameString},
    loaded::{AssetSequenceIdMappings, SequenceId},
};

/// Maps `SequenceId`s from sequence strings.
#[derive(Debug)]
pub struct SequenceIdMapper<'s, SeqName>
where
    SeqName: SequenceName,
{
    /// `AssetSequenceIdMappings<SeqName>`.
    pub asset_sequence_id_mappings: &'s AssetSequenceIdMappings<SeqName>,
}

impl<'s, SeqName> SequenceIdMapper<'s, SeqName>
where
    SeqName: SequenceName,
{
    /// Maps items to `SequenceId`.
    pub fn strings_to_ids<SequenceStringIterator, SequenceStringRef>(
        &self,
        asset_slug: &AssetSlug,
        sequence_string_iterator: SequenceStringIterator,
        asset_id: AssetId,
    ) -> Vec<SequenceId>
    where
        SequenceStringIterator: Iterator<Item = SequenceStringRef>,
        SequenceStringRef: AsRef<str>,
    {
        sequence_string_iterator
            .map(|sequence_string| {
                let sequence_string = AsRef::<str>::as_ref(&sequence_string);
                self.string_to_id(asset_slug, sequence_string, asset_id)
            })
            .collect::<Vec<SequenceId>>()
    }

    /// Maps a `&str` to a `SequenceId`.
    pub fn string_to_id(
        &self,
        asset_slug: &AssetSlug,
        sequence_string: &str,
        asset_id: AssetId,
    ) -> SequenceId {
        let sequence_name_string = SequenceNameString::from_str(sequence_string)
            .expect("Expected `SequenceNameString::from_str` to succeed.");

        self.item_to_data(asset_slug, &sequence_name_string, asset_id)
    }

    /// Maps items to `SequenceId`.
    pub fn items_to_datas<'f, SequenceNameStringIterator>(
        &self,
        asset_slug: &AssetSlug,
        sequence_name_string_iterator: SequenceNameStringIterator,
        asset_id: AssetId,
    ) -> Vec<SequenceId>
    where
        SequenceNameStringIterator: Iterator<Item = &'f SequenceNameString<SeqName>>,
    {
        sequence_name_string_iterator
            .map(|sequence_name_string| {
                self.item_to_data(asset_slug, sequence_name_string, asset_id)
            })
            .collect::<Vec<SequenceId>>()
    }

    /// Maps a `SequenceNameString<SeqName>` to a `SequenceId`.
    pub fn item_to_data(
        &self,
        asset_slug: &AssetSlug,
        sequence_name_string: &SequenceNameString<SeqName>,
        asset_id: AssetId,
    ) -> SequenceId {
        let sequence_id_mappings = self
            .asset_sequence_id_mappings
            .get(asset_id)
            .expect("Expected `SequenceIdMapping` to be loaded.");
        sequence_id_mappings
            .id(&sequence_name_string)
            .copied()
            .unwrap_or_else(|| {
                error!(
                    "`{}` sequence ID not found for asset: `{}`. \
                     Falling back to first declared sequence.",
                    sequence_name_string, asset_slug
                );

                SequenceId::new(0)
            })
    }
}
