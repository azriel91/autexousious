use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Display, From, Not,
};

/// Whether the object is mirrored.
#[derive(
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    From,
    PartialEq,
    Eq,
    Not,
)]
pub struct Mirrored(pub bool);
