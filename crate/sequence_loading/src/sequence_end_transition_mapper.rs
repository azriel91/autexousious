use std::{marker::PhantomData, str::FromStr};

use asset_model::config::AssetSlug;
use sequence_model::{
    config::{self, Sequence, SequenceName, SequenceNameString, Wait},
    loaded::{SequenceEndTransition, SequenceIdMappings},
};

/// Maps `config::SequenceEndTransition` into `loaded::SequenceEndTransition`.
#[derive(Debug)]
pub struct SequenceEndTransitionMapper<SeqName>
where
    SeqName: SequenceName,
{
    /// Marker.
    marker: PhantomData<SeqName>,
}

impl<SeqName> SequenceEndTransitionMapper<SeqName>
where
    SeqName: SequenceName,
{
    /// Maps a `config::SequenceEndTransition` to `loaded::SequenceEndTransition`.
    pub fn map<SequenceRef, Frm>(
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        asset_slug: &AssetSlug,
        sequence_ref: SequenceRef,
    ) -> SequenceEndTransition
    where
        SequenceRef: AsRef<Sequence<SeqName, Frm>>,
        Frm: AsRef<Wait>,
    {
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
                            "Invalid sequence specified: `{}` for asset `{}`.",
                            sequence_name_string, asset_slug
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
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        asset_slug: &AssetSlug,
        sequence_ref: SequenceRef,
    ) -> SequenceEndTransition
    where
        SequenceRef: AsRef<Sequence<SeqNameLocal, Frm>>,
        SeqNameLocal: SequenceName,
        Frm: AsRef<Wait>,
    {
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
                            "Invalid sequence specified: `{}` for asset `{}`.",
                            sequence_name_string, asset_slug
                        )
                    });
                SequenceEndTransition::SequenceId(sequence_id)
            }
        }
    }
}
