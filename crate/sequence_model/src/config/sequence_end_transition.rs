use std::{fmt, marker::PhantomData, str::FromStr};

use derivative::Derivative;
use derive_new::new;
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use strum_macros::{Display, EnumString, IntoStaticStr};
use typename_derive::TypeName;

use crate::config::SequenceId;

/// Specifies the behaviour to transition when the sequence ends.
#[derive(
    Clone, Copy, Debug, Derivative, Display, EnumString, IntoStaticStr, PartialEq, TypeName,
)]
#[derivative(Default)]
#[strum(serialize_all = "snake_case")]
pub enum SequenceEndTransition<SeqId>
where
    SeqId: SequenceId,
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
    // TODO: Ideally we could use `#[serde(flatten)]` for enum variants, but it isn't supported yet.
    // TODO: See: <https://github.com/serde-rs/serde/issues/1402>
    SequenceId(SeqId),
}

impl<SeqId> Serialize for SequenceEndTransition<SeqId>
where
    SeqId: SequenceId,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let enum_name = stringify!(SequenceEndTransition);
        match self {
            SequenceEndTransition::None => {
                let variant_index = 0;
                let variant_name = Into::<&'static str>::into(SequenceEndTransition::<SeqId>::None);
                serializer.serialize_unit_variant(enum_name, variant_index, &variant_name)
            }
            SequenceEndTransition::Repeat => {
                let variant_index = 1;
                let variant_name =
                    Into::<&'static str>::into(SequenceEndTransition::<SeqId>::Repeat);
                serializer.serialize_unit_variant(enum_name, variant_index, &variant_name)
            }
            SequenceEndTransition::Delete => {
                let variant_index = 2;
                let variant_name =
                    Into::<&'static str>::into(SequenceEndTransition::<SeqId>::Delete);
                serializer.serialize_unit_variant(enum_name, variant_index, &variant_name)
            }
            SequenceEndTransition::SequenceId(sequence_id) => {
                let variant_index = 3;
                let variant_name = Into::<&'static str>::into(*sequence_id);
                serializer.serialize_unit_variant(enum_name, variant_index, &variant_name)
            }
        }
    }
}

impl<'de, SeqId> Deserialize<'de> for SequenceEndTransition<SeqId>
where
    SeqId: SequenceId,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SequenceEndTransitionVisitor::new())
    }
}

#[derive(new)]
struct SequenceEndTransitionVisitor<SeqId>(PhantomData<SeqId>)
where
    SeqId: SequenceId;

impl<'de, SeqId> Visitor<'de> for SequenceEndTransitionVisitor<SeqId>
where
    SeqId: SequenceId,
{
    type Value = SequenceEndTransition<SeqId>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("one of `none`, `repeat`, `delete`, or a sequence ID")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        SequenceEndTransition::from_str(value)
            .or_else(|_| SeqId::from_str(value).map(SequenceEndTransition::SequenceId))
            .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
    }
}
