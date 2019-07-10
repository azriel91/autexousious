use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

use amethyst::ecs::Component;

/// Marker trait for everywhere that uses sequence IDs.
///
/// TODO: RFC 1733 will allow us to define an alias instead of a new trait. See:
///
/// * <https://github.com/rust-lang/rfcs/blob/master/text/1733-trait-alias.md>
/// * <https://github.com/rust-lang/rust/issues/41517>
pub trait SequenceId:
    Component
    + Copy
    + Debug
    + Default
    + Display
    + Eq
    + FromStr
    + Into<&'static str>
    + Hash
    + Send
    + Sync
{
}
