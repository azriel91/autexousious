use std::convert::AsRef;

use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{Frame, SequenceEndTransition, SequenceName, Wait};

/// A sequence of frames that represents independent behaviour.
///
/// Sequences are shared by different object types, and are genericized by the
/// sequence name. This is because different object types have different valid
/// sequence names, and we want to be able to define this at compile time rather
/// than needing to process this at runtime.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct Sequence<SeqName, Frm = Frame>
where
    SeqName: SequenceName,
    Frm: AsRef<Wait>,
{
    /// Name of the sequence to switch to after this one has completed.
    pub next: SequenceEndTransition<SeqName>,
    /// Frames in this sequence.
    pub frames: Vec<Frm>,
}

impl<SeqName, Frm> AsRef<Sequence<SeqName, Frm>> for Sequence<SeqName, Frm>
where
    SeqName: SequenceName,
    Frm: AsRef<Wait>,
{
    fn as_ref(&self) -> &Sequence<SeqName, Frm> {
        self
    }
}
