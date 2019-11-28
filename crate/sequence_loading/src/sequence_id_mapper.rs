use std::{marker::PhantomData, str::FromStr};

use asset_model::config::AssetSlug;
use log::error;
use sequence_model::{
    config::{SequenceName, SequenceNameString},
    loaded::{SequenceId, SequenceIdMappings},
};

/// Maps `SequenceId`s from sequence strings.
#[derive(Debug)]
pub struct SequenceIdMapper<SeqName>
where
    SeqName: SequenceName,
{
    /// Marker.
    marker: PhantomData<SeqName>,
}

impl<SeqName> SequenceIdMapper<SeqName>
where
    SeqName: SequenceName,
{
    /// Returns a new SequenceIdMapper.
    pub fn new() -> Self {
        SequenceIdMapper {
            marker: PhantomData,
        }
    }

    /// Maps items to `SequenceId`.
    pub fn strings_to_ids<SequenceStringIterator, SequenceStringRef>(
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        asset_slug: &AssetSlug,
        sequence_string_iterator: SequenceStringIterator,
    ) -> Vec<SequenceId>
    where
        SequenceStringIterator: Iterator<Item = SequenceStringRef>,
        SequenceStringRef: AsRef<str>,
    {
        sequence_string_iterator
            .map(|sequence_string| {
                let sequence_string = AsRef::<str>::as_ref(&sequence_string);
                Self::string_to_id(sequence_id_mappings, asset_slug, sequence_string)
            })
            .collect::<Vec<SequenceId>>()
    }

    /// Maps a `&str` to a `SequenceId`.
    pub fn string_to_id(
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        asset_slug: &AssetSlug,
        sequence_string: &str,
    ) -> SequenceId {
        let sequence_name_string = SequenceNameString::from_str(sequence_string)
            .expect("Expected `SequenceNameString::from_str` to succeed.");

        Self::item_to_data(sequence_id_mappings, asset_slug, &sequence_name_string)
    }

    /// Maps items to `SequenceId`.
    pub fn items_to_datas<'f, SequenceNameStringIterator>(
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        asset_slug: &AssetSlug,
        sequence_name_string_iterator: SequenceNameStringIterator,
    ) -> Vec<SequenceId>
    where
        SequenceNameStringIterator: Iterator<Item = &'f SequenceNameString<SeqName>>,
    {
        sequence_name_string_iterator
            .map(|sequence_name_string| {
                Self::item_to_data(sequence_id_mappings, asset_slug, sequence_name_string)
            })
            .collect::<Vec<SequenceId>>()
    }

    /// Maps a `SequenceNameString<SeqName>` to a `SequenceId`.
    pub fn item_to_data(
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        asset_slug: &AssetSlug,
        sequence_name_string: &SequenceNameString<SeqName>,
    ) -> SequenceId {
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
