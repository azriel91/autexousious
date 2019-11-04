#![allow(missing_debug_implementations)] // Needed for derived `EnumIter`

use std::convert::TryFrom;

use object_type::ObjectType;
use strum_macros::{Display, EnumDiscriminants, EnumIter};

/// Game configuration types.
///
/// Allows compile-time checks for ensuring all configuration types are discovered.
#[derive(Clone, Copy, Debug, EnumDiscriminants, Hash, PartialEq, Eq)]
#[strum_discriminants(
    derive(Display, EnumIter, Hash),
    name(AssetTypeVariant),
    strum(serialize_all = "snake_case")
)]
pub enum AssetType {
    /// Things that can be interacted with in-game.
    Object(ObjectType),
    /// Playing field for objects.
    Map,
    /// User interface assets.
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
