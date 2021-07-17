use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

/// Marker trait for everywhere that uses sequence names.
///
/// Sequence names may be well-known (enum variant), or arbitrary (fallback enum
/// variant that holds the string).
///
/// TODO: RFC 1733 will allow us to define an alias instead of a new trait. See:
///
/// * <https://github.com/rust-lang/rfcs/blob/master/text/1733-trait-alias.md>
/// * <https://github.com/rust-lang/rust/issues/41517>
pub trait SequenceName:
    Copy + Debug + Default + Display + Eq + FromStr + Into<&'static str> + Hash + Send + Sync + 'static
{
}
