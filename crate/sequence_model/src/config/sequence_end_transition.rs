use std::{fmt, marker::PhantomData, str::FromStr};

use derivative::Derivative;
use derive_new::new;
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use strum_macros::{Display, EnumString, IntoStaticStr};

use crate::config::{SequenceName, SequenceNameString};

/// Specifies the behaviour to transition when the sequence ends.
#[derive(Clone, Debug, Derivative, Display, EnumString, IntoStaticStr, PartialEq)]
#[derivative(Default)]
#[strum(serialize_all = "snake_case")]
pub enum SequenceEndTransition<SeqName>
where
    SeqName: SequenceName,
{
    /// Don't transition, stay on the last frame.
    #[derivative(Default)]
    None,
    /// Repeat the current sequence.
    Repeat,
    /// Delete the object after the sequence has ended.
    Delete,
    /// Transition to the specified sequence.
    //
    // TODO: Ideally we could use `#[serde(flatten)]` for enum variants, but it isn't supported
    // yet. TODO: See: <https://github.com/serde-rs/serde/issues/1402>
    SequenceName(SequenceNameString<SeqName>),
}

impl<SeqName> Serialize for SequenceEndTransition<SeqName>
where
    SeqName: SequenceName,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let enum_name = stringify!(SequenceEndTransition);
        match self {
            SequenceEndTransition::None => {
                let variant_index = 0;
                let variant_name =
                    Into::<&'static str>::into(SequenceEndTransition::<SeqName>::None);
                serializer.serialize_unit_variant(enum_name, variant_index, variant_name)
            }
            SequenceEndTransition::Repeat => {
                let variant_index = 1;
                let variant_name =
                    Into::<&'static str>::into(SequenceEndTransition::<SeqName>::Repeat);
                serializer.serialize_unit_variant(enum_name, variant_index, variant_name)
            }
            SequenceEndTransition::Delete => {
                let variant_index = 2;
                let variant_name =
                    Into::<&'static str>::into(SequenceEndTransition::<SeqName>::Delete);
                serializer.serialize_unit_variant(enum_name, variant_index, variant_name)
            }
            SequenceEndTransition::SequenceName(sequence_name_string) => {
                let string = match sequence_name_string {
                    SequenceNameString::Name(sequence_name) => {
                        Into::<&'static str>::into(*sequence_name)
                    }
                    SequenceNameString::String(string) => string,
                };
                serializer.serialize_str(string)
            }
        }
    }
}

impl<'de, SeqName> Deserialize<'de> for SequenceEndTransition<SeqName>
where
    SeqName: SequenceName,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SequenceEndTransitionVisitor::new())
    }
}

#[derive(new)]
struct SequenceEndTransitionVisitor<SeqName>(PhantomData<SeqName>)
where
    SeqName: SequenceName;

impl<'de, SeqName> Visitor<'de> for SequenceEndTransitionVisitor<SeqName>
where
    SeqName: SequenceName,
{
    type Value = SequenceEndTransition<SeqName>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("one of `none`, `repeat`, `delete`, or a sequence ID")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        SequenceEndTransition::from_str(value)
            .or_else(|_| {
                SequenceNameString::from_str(value).map(SequenceEndTransition::SequenceName)
            })
            .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
    }
}
