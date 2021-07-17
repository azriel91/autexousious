use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};

/// Newtype for counting independent team number.
///
/// The value this holds should be valid when retrieved. If the value is
/// assigned to a new `Independent` team, then it must be incremented after. The
/// `#get_and_increment` method is provided to make it easier to uphold this
/// contract.
#[numeric_newtype]
#[derive(Debug, Default, Deserialize, Hash, Serialize)]
pub struct IndependentCounter(pub u32);

impl IndependentCounter {
    /// Returns a copy of the current counter value, and increments itself.
    pub fn get_and_increment(&mut self) -> Self {
        let current_value = *self;
        self.0 += 1;
        current_value
    }
}
