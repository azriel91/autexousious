use std::any::type_name;

use asset_model::loaded::AssetId;
use sequence_model::{
    config::{self, Sequence, SequenceName, Wait},
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
            config::SequenceEndTransition::SequenceName(sequence_name) => {
                let sequence_id = sequence_id_mappings
                    .id(sequence_name)
                    .copied()
                    .unwrap_or_else(|| {
                        panic!(
                            "Invalid sequence specified: `{}` for asset ID: `{:?}`.",
                            sequence_name, asset_id
                        )
                    });
                SequenceEndTransition::SequenceId(sequence_id)
            }
        }
    }
}
