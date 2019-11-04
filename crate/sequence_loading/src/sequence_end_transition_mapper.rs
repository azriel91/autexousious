use std::{any::type_name, str::FromStr};

use asset_model::loaded::AssetId;
use sequence_model::{
    config::{self, Sequence, SequenceName, SequenceNameString, Wait},
    loaded::{AssetSequenceIdMappings, SequenceEndTransition},
};

/// Maps `config::SequenceEndTransition` into `loaded::SequenceEndTransition`.
#[derive(Debug)]
pub struct SequenceEndTransitionMapper<'s, SeqName>
where
    SeqName: SequenceName,
{
    /// `AssetSequenceIdMappings<SeqName>`.
    pub asset_sequence_id_mappings: &'s AssetSequenceIdMappings<SeqName>,
}

impl<'s, SeqName> SequenceEndTransitionMapper<'s, SeqName>
where
    SeqName: SequenceName,
{
    /// Maps a `config::SequenceEndTransition` to `loaded::SequenceEndTransition`.
    pub fn map<SequenceRef, Frm>(
        &self,
        asset_id: AssetId,
        sequence_ref: SequenceRef,
    ) -> SequenceEndTransition
    where
        SequenceRef: AsRef<Sequence<SeqName, Frm>>,
        Frm: AsRef<Wait>,
    {
        let sequence_id_mappings = self
            .asset_sequence_id_mappings
            .get(asset_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SequenceIdMappings<{}>` to exist for asset ID: `{:?}`.",
                    type_name::<SeqName>(),
                    asset_id
                )
            });
        let sequence = AsRef::<Sequence<SeqName, Frm>>::as_ref(&sequence_ref);

        match &sequence.next {
            config::SequenceEndTransition::None => SequenceEndTransition::None,
            config::SequenceEndTransition::Repeat => SequenceEndTransition::Repeat,
            config::SequenceEndTransition::Delete => SequenceEndTransition::Delete,
            config::SequenceEndTransition::SequenceName(sequence_name_string) => {
                let sequence_id = sequence_id_mappings
                    .id(sequence_name_string)
                    .copied()
                    .unwrap_or_else(|| {
                        panic!(
                            "Invalid sequence specified: `{}` for asset ID: `{:?}`.",
                            sequence_name_string, asset_id
                        )
                    });
                SequenceEndTransition::SequenceId(sequence_id)
            }
        }
    }

    /// Maps a `config::SequenceEndTransition<T>` to `loaded::SequenceEndTransition`.
    ///
    /// This method differs from [`SequenceEndTransitionMapper::map`] as it accepts a `Sequence`
    /// with a different `SequenceName`.
    ///
    /// # Panics
    ///
    /// Panics if the sequence end transition is a `SequenceName` instead of a `String`.
    pub fn map_disparate<SequenceRef, SeqNameLocal, Frm>(
        &self,
        asset_id: AssetId,
        sequence_ref: SequenceRef,
    ) -> SequenceEndTransition
    where
        SequenceRef: AsRef<Sequence<SeqNameLocal, Frm>>,
        SeqNameLocal: SequenceName,
        Frm: AsRef<Wait>,
    {
        let sequence_id_mappings = self
            .asset_sequence_id_mappings
            .get(asset_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SequenceIdMappings<{}>` to exist for asset ID: `{:?}`.",
                    type_name::<SeqName>(),
                    asset_id
                )
            });
        let sequence = AsRef::<Sequence<SeqNameLocal, Frm>>::as_ref(&sequence_ref);

        match &sequence.next {
            config::SequenceEndTransition::None => SequenceEndTransition::None,
            config::SequenceEndTransition::Repeat => SequenceEndTransition::Repeat,
            config::SequenceEndTransition::Delete => SequenceEndTransition::Delete,
            config::SequenceEndTransition::SequenceName(sequence_name_string) => {
                let sequence_name_string = match sequence_name_string {
                    SequenceNameString::Name(sequence_name) => panic!(
                        "Failed to map disparate sequence end transition. \
                         Encountered sequence ID: `{}`.",
                        sequence_name
                    ),
                    SequenceNameString::String(sequence_string) => {
                        SequenceNameString::from_str(sequence_string)
                            .expect("Expected `SequenceNameString::from_str` to succeed.")
                    }
                };
                let sequence_id = sequence_id_mappings
                    .id(&sequence_name_string)
                    .copied()
                    .unwrap_or_else(|| {
                        panic!(
                            "Invalid sequence specified: `{}` for asset ID: `{:?}`.",
                            sequence_name_string, asset_id
                        )
                    });
                SequenceEndTransition::SequenceId(sequence_id)
            }
        }
    }
}
