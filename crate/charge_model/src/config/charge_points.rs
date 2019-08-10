use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};

/// Charge points of an object.
#[numeric_newtype]
#[derive(Debug, Default, Deserialize, Hash, Serialize)]
pub struct ChargePoints(pub u32);
