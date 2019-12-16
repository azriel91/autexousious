use smallvec::SmallVec;

use crate::config::InputReactionAppEvent;

/// Alias so that size does not need to be updated downstream.
pub type InputReactionAppEvents = SmallVec<[InputReactionAppEvent; 1]>;
