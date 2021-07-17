use std::{fmt, str::FromStr};

use derivative::Derivative;
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use strum_macros::{Display, EnumString, IntoStaticStr};

/// Number of objects a `Hit` may collide with.
///
/// Serialization and deserialization for this type is custom, so users may
/// specify one of the following:
///
/// ```yaml
/// hit_limit: 2            # HitLimit::Limit(2)
/// hit_limit: "unlimited"  # HitLimit::Unlimited
/// ```
#[derive(
    Clone, Copy, Debug, Derivative, Display, EnumString, IntoStaticStr, Hash, PartialEq, Eq,
)]
#[derivative(Default)]
#[strum(serialize_all = "snake_case")]
pub enum HitLimit {
    /// Limit to `n` objects.
    #[derivative(Default)]
    Limit(#[derivative(Default(value = "1"))] u32),
    /// Not limited.
    Unlimited,
}

impl Serialize for HitLimit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            HitLimit::Limit(limit) => serializer.serialize_u32(*limit),
            HitLimit::Unlimited => {
                let enum_name = stringify!(HitLimit);
                let variant_index = 1;
                let variant_name = Into::<&'static str>::into(HitLimit::Unlimited);
                serializer.serialize_unit_variant(enum_name, variant_index, &variant_name)
            }
        }
    }
}

impl<'de> Deserialize<'de> for HitLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(HitLimitVisitor)
    }
}

struct HitLimitVisitor;

macro_rules! impl_visit_numeric {
    ($visit_name:ident, $ty:ident) => {
        fn $visit_name<E>(self, value: $ty) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if value >= $ty::from(std::u32::MIN) && value <= $ty::from(std::u32::MAX) {
                Ok(HitLimit::Limit(value as u32))
            } else {
                Err(E::custom(format!("u32 out of range: {}", value)))
            }
        }
    };
}

impl<'de> Visitor<'de> for HitLimitVisitor {
    type Value = HitLimit;

    impl_visit_numeric!(visit_i64, i64);

    impl_visit_numeric!(visit_i128, i128);

    impl_visit_numeric!(visit_u64, u64);

    impl_visit_numeric!(visit_u128, u128);

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let variant_unlimited: &str = HitLimit::Unlimited.into();
        formatter.write_str("a u32 or `\"")?;
        formatter.write_str(variant_unlimited)?;
        formatter.write_str("\"`")
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(HitLimit::Limit(value))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if value >= 0 {
            Ok(HitLimit::Limit(value as u32))
        } else {
            Err(E::custom(format!("u32 out of range: {}", value)))
        }
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        HitLimit::from_str(value).map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
    }
}
