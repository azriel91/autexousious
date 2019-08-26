use std::fmt::{self, Display};

use std::{convert::Infallible, str::FromStr};

use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::config::SequenceName;

/// Enclosing type for either a known sequence name, or an arbitrary string.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[derivative(Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case", untagged)]
pub enum SequenceNameString<SeqName>
where
    SeqName: SequenceName,
{
    /// A known sequence name.
    #[derivative(Default)]
    Name(SeqName),
    /// An arbitrary string.
    String(String),
}

impl<SeqName> Display for SequenceNameString<SeqName>
where
    SeqName: SequenceName,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SequenceNameString::Name(sequence_name) => write!(f, "{}", sequence_name),
            SequenceNameString::String(sequence_string) => write!(f, "{}", sequence_string),
        }
    }
}

impl<SeqName> From<SeqName> for SequenceNameString<SeqName>
where
    SeqName: SequenceName,
{
    fn from(sequence_name: SeqName) -> Self {
        SequenceNameString::Name(sequence_name)
    }
}

impl<SeqName> FromStr for SequenceNameString<SeqName>
where
    SeqName: SequenceName,
{
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Infallible> {
        SeqName::from_str(s)
            .map(SequenceNameString::Name)
            .or_else(|_e| {
                let string = String::from(s);
                Ok(SequenceNameString::String(string))
            })
    }
}
