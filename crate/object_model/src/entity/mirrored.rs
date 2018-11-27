use amethyst::ecs::{storage::VecStorage, Component};
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

/// Not every entity will have this, but since it's simply a `bool` wrapper, the indirection table
/// actually uses more space.
impl Component for Mirrored {
    type Storage = VecStorage<Self>;
}
