use std::marker::PhantomData;

use derivative::Derivative;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::config::{Sequence, SequenceName, SequenceNameString, Wait};

/// Mappings of `SequenceName` to `Sequence`s.
///
/// For some reason, when using a tuple struct, serde does not transparently deserialize the inner
/// `IndexMap` (tested through `UiSequences`, which is a type alias of this).
/// Also, serde requires us to specify `#[serde(skip)]` on the `marker` field instead of
/// automatically using a default value.
///
/// See <https://github.com/serde-rs/serde/issues/1660>.
#[derive(Clone, Debug, Deref, DerefMut, Derivative, Deserialize, PartialEq, Serialize, new)]
#[derivative(Default(bound = ""))] // Don't require `Seq: Default`
pub struct Sequences<Seq, SeqName, Frm>
where
    Seq: AsRef<Sequence<SeqName, Frm>>,
    SeqName: SequenceName,
    Frm: AsRef<Wait>,
{
    /// Map of sequence name string to sequence.
    #[serde(flatten)]
    pub sequences: IndexMap<SequenceNameString<SeqName>, Seq>,
    /// Marker.
    #[serde(skip)]
    marker: PhantomData<Frm>,
}
