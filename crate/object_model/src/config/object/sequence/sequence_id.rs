use std::fmt::Debug;
use std::hash::Hash;

/// Marker trait for everywhere that uses sequence IDs.
pub trait SequenceId: Copy + Debug + Eq + Hash + Send + Sync {}
