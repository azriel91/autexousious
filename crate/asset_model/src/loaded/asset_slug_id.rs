use amethyst::ecs::{storage::VecStorage, Component};
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Asset slug ID.
///
/// This is a cheap `Copy` type to use instead of `AssetSlug` which is `Clone`.
#[numeric_newtype]
#[derive(Component, Debug, Deserialize, Hash, Serialize)]
#[storage(VecStorage)]
pub struct AssetSlugId(pub usize);
