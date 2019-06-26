use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};

/// Newtype for counting numbered teams.
#[numeric_newtype]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TeamCounter(pub u32);
