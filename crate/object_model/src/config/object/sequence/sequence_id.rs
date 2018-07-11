use std::fmt::Debug;
use std::hash::Hash;

/// Marker trait for everywhere that uses sequence IDs.
pub trait SequenceId: Copy + Debug + Default + Eq + Hash + Send + Sync {}
