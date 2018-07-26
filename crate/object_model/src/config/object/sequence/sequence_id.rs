use std::fmt::Debug;
use std::hash::Hash;

/// Marker trait for everywhere that uses sequence IDs.
///
/// TODO: RFC 1733 will allow us to define an alias instead of a new trait. See:
///
/// * <https://github.com/rust-lang/rfcs/blob/master/text/1733-trait-alias.md>
/// * <https://github.com/rust-lang/rust/issues/41517>
pub trait SequenceId: Copy + Debug + Default + Eq + Hash + Send + Sync {}
