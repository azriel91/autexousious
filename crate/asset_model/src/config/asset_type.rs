#![allow(missing_debug_implementations)] // Needed for derived `EnumIter`

//! Provides the `AssetType` enum.

use std::convert::TryFrom;

use enum_variant_type::EnumVariantType;
use object_type::ObjectType;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};

/// Game configuration types.
///
/// Allows compile-time checks for ensuring all configuration types are
/// discovered.
#[derive(Clone, Copy, Debug, EnumDiscriminants, EnumVariantType, Hash, PartialEq, Eq)]
#[strum_discriminants(
    derive(Display, EnumIter, Hash),
    name(AssetTypeVariant),
    strum(serialize_all = "snake_case")
)]
pub enum AssetType {
    /// Things that can be interacted with in-game.
    #[evt(skip)]
    Object(ObjectType),
    /// Playing field for objects.
    #[evt(derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize))]
    Map,
    /// User interface assets.
    #[evt(skip)]
    Ui,
}

impl TryFrom<AssetTypeVariant> for AssetType {
    type Error = AssetTypeVariant;

    fn try_from(asset_type_variant: AssetTypeVariant) -> Result<AssetType, AssetTypeVariant> {
        match asset_type_variant {
            AssetTypeVariant::Object => Err(asset_type_variant),
            AssetTypeVariant::Map => Ok(AssetType::Map),
            AssetTypeVariant::Ui => Ok(AssetType::Ui),
        }
    }
}
